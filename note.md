In my understanding, the popular development workflow is SDD (Spec-Driven Development), and the tools we are using now are OpenSpec + Claude UI + MiniMax 2.5.
This prototyping will not involve multi-agents doing separate works and then integrating. Instead, we define and iterate on the spec, which gives the AI agent exact instructions on what to do.

Clarification for this tournament: Since our ZKVM implementation currently lacks true cryptographic privacy features, strict privacy is not required. However, we should implement the specific operation parameters as private inputs to demonstrate and convey the architectural intent of privacy-preserving media editing.

关于 Pico ZKVM 处理 C2PA 大图哈希性能问题的反馈与方案建议
1. 现状痛点 (Echoing Elliot & Jie Tang): 在 ZKVM (RISC-V) 内部对 3MB+ 的原图进行全量 SHA-256 哈希确实会导致处理周期（Cycles）爆炸。在 Pico 没有 SHA-256 Precompile（硬件加速）的情况下，单机模拟确实很难跑出来。

2. 我们的避坑方案: Development Scaling (开发环境缩放协议) 为了确保端到端流程能跑通且可演示，建议采用以下“分层验证”策略：

实施方法：我们在 mock-signer 签名阶段，并没有直接哈希全量原始像素，而是先将图片降采样至 256x256 (RGB8)，然后再进行 P-256 签名。
计算量对比：
3MB 原图：哈希开销极大，可能触发 OOM 或超长等待。
256x256 图片：数据量约 192KB。在 Pico 里的总周期约为 24 Million Cycles。这个量级在普通笔记本 CPU 上用 emulate 模式可以在秒级跑完，逻辑验证非常丝滑。
安全合规性建议：在实际 C2PA 行业标准中，相机硬件本身也经常对内容的低分辨率哈希 (Lower-res hash) 或 预览图断言 (Thumbnail Assertion) 进行签名。因此，在 ZK 证明中验证一个“采样后的签名图”，在逻辑完备性上与原图是一致的。
3. 对生产部署的思考 (Path to Production):

Demo 阶段：坚持使用 256x256 或 512x512 的采样比例。这能保证我们的端到端闭环（签名验证 + 像素编辑重放 + 输出哈希比对）是 0 Mock 运行的。
生产阶段：一旦上线 GPU Server 或 Pico 更新了 SHA-256 Precompile，我们只需将 mock-signer 里的 thumbnail 缩放比例改回原图尺寸即可，底层的 ZK 电路逻辑完全不需要改动。
总结建议： 建议大家先把“像素粒度”降下来，跑通 “签名自洽 + 编辑重放” 的完整逻辑。性能是个工程问题（靠 GPU/加速），但逻辑闭环才是我们 ZK-C2PA 项目的核心竞争力。这样大家就都能在本机跑出 Proof 了。


Jie Tang  [9:16 AM]
感觉可能不是hash的问题，是不是ecdsa p256的身份验证是bottleneck？但是这里目前似乎只能用mock-signer来做，假设现在是sony设备签的，换成尼康或者其他根证书签的，不是又无法验证了，身份验证放到zkvm里面似乎不太好解决。 (edited) 
Jinyao  [9:24 AM]
另外感觉是我vibe coding提示不对(方向错误)，生成出来的code总是想把hash send给zkVM, 这种approach会存在security问题吗？
However, we don't actually need to send the full image to the zkVM. Instead, we use the hash from the C2PA metadata (the claim_hash field) as a commitment to the original image. This is a common pattern in ZK proofs - proving knowledge of something without revealing it.

Here's the current flow:

Prover has the full original image (9MB) and extracts C2PA metadata including the claim_hash
Prover applies image operations and creates the final image
Prover sends to zkVM:
claim_hash (32 bytes) - from C2PA metadata
Operations (small parameters like crop coordinates, resize dimensions)
Final image hash (32 bytes) - computed from the output file
zkVM computes a commitment: H(claim_hash || operations) and outputs this as public value
The zkVM doesn't need the full image - it only needs the hash commitment from C2PA. The security relies on the C2PA signature being valid (proving the original image was authentic), and the ZK proof proving that the operations were applied to produce the final image hash.Victor  [9:25 AM]
replied to a thread:可能需要一个ecdsa p256的precompile :joy:
Jie Tang  [9:28 AM]
之前加过这个patch: https://github.com/brevis-network/signatures curve得选取下p256
Victor  [9:30 AM]
replied to a thread:@Alan secp256r1 precompile我们支持吗？secp256k1我知道是work的
Jie Tang  [9:32 AM]
不过我刚才的意思是，这可能是个产品问题，我们现在vm里面内置一个public key来做签名的验证，但是其他设备签的图片就verify不了了。另外一个方案就是把public key作为public input从前端传入，verify的时候vm去获取来验证，但是但是UI就有点confused。

否则只能mock了 (edited) 
Mo Dong  [9:33 AM]
replied to a thread:尼康 索尼 根证书都可以认为是public input和public knowledge
Mo Dong  [9:36 AM]
replied to a thread:你说的没错，不同设备制造商的public key是一个public information，也要作为ZKVM的public input的一部分，你想，将来的验证flow就是，索尼的设备用索尼的根证书验证，尼康的用尼康的，莱卡用莱卡的，而这些根证书，都假设是所有人都知道并可以验证是他们的根证书的
Jie Tang  [9:37 AM]
ok 那就没问题了
Elliott  [9:44 AM]
我怎么还是感觉设备签名不需要在zkvm里验证啊？zkvm只需要证明一个文件经过几次操作变成另一个文件，然后output commit里有这两个文件的hash，通过一个权威网站可以查询原文件hash的有效性，然后编辑后的文件和后一个hash一致，不就可以了么？
Jie Tang  [9:45 AM]
权威网站也解决不了来源可信的问题呀
Elliott  [9:46 AM]
权威网站里就是验证原文件的签名啊，这个任何人有文件都可以验证，不需要zk proof额外再证一次
Jie Tang  [9:46 AM]
他们都中心化的验证
[9:47 AM]跟我们自己 host server 验证就没区别了 (edited) 
Elliott  [9:49 AM]
zk proof一样的道理啊，可以我们验证，也可以别人验证。我的意思是图片原文件本身就包含自证信息。我们zkvm所需要确定的一点是编辑的起点文件是来自于正确的文件，这个通过hash关联就可以
Mo Dong  [9:52 AM]
replied to a thread:我认为你说的可能是对的，但我没看具体的签名细节 所以不完全确定

我的理解是

相机签名，目前签名的是一个相片的metadata，这个metadata里面有啥，我没仔细研究过，那既然签名的是metadata，这个metadata里面是不是有hash，这个hash是不是和ZKVM的public input的hash是一样的。如果是一样的，那也不需要什么网站了，直接扔一个证书信任的key签出来的metadata就好了。验证者拿到的是 metadata，metadata的签名，照片编辑这个计算的zkproof。他要干两件事，1. 验证metadata签名X509 cert chain是对的，这个过程中要访问sony的根证书，然后验证；2. 验证计算是对的，这个靠ZK Proof。

如果确实是这样的话，在verifier不是blockchain的时候，我的理解是，这个第一步 是不是在VM里面做是无所谓的。

但是当verifier是一个blockchain smart contract的时候，你就需要把所有东西都包进去了，否则onchain也算不了那个sig check。
Elliott  [9:54 AM]
smart contract的话确实应该是都包含
Mo Dong  [9:55 AM]
你上午第一次问这个问题的时候，我的回答就是，命题作文的基础要求可以不假设需要smart contract验证，但是需要确定这个信任链条是work的
[9:57 AM]这个问题 不能假设 需要研究并且有研究的proof
Elliott  [9:57 AM]
我现在有另一个问题，我记得老董说用户验证的时候只需要提供编辑后的文件，以及被编辑的步骤。我们喂给zkvm private input的原文件是怎么找出来的？
Mo Dong  [9:57 AM]
没懂这个问题是啥
Elliott  [9:59 AM]
你想想啊，我们有个界面，用户去验证一个被编辑过后的文件的真实性，他需要提供最开始的那个设备输出的源文件么？ (edited) 
Mo Dong  [10:00 AM]
不需要源文件
[10:00 AM]建议研究zkvm的工作原理
[10:00 AM]可以问下AI
[10:00 AM]verfier需要啥 不需要啥
[10:01 AM]我可以说这不属于产品问题 产品需求是可以实现的，工程问题我就不回答了 (edited) 
Elliott  [10:06 AM]
哦，我知道了，我搞错了。是编辑者在编辑后就跑zkvm program生成了证明，这个proof被比如smart contract验证后，源文件和编辑后的文件的hash映射就保存在onchain，当然源文件的metadata也可以输出存在onchain(如有查询必要)。后边的verifier只需要直接提供编辑后的文件，我们算下这个文件hash，onchain存在valid的映射即可


@channel UPDATE: A C2PA signed image is now available at  https://drive.google.com/drive/folders/1oqat4phmkn7uBToAEx_Te1b0C8ejlA2g?usp=sharing

You can check its signature validity at https://digitalsignatureself-checker.authenticity.sony.net/page/digitalSignatureSelfChecker (edited) 
Venus  [9:30 PM]
joined #vera-vibe.Jinyao  [9:31 PM]
image.png Venus  [9:31 PM]
left #vera-vibe.Elliott  [9:32 PM]
every team will present their workflow, tooling setup, and building process to the entire team. We want to learn not only what you built, but how you built it
Elliott  [9:39 PM]
in my understanding, the popular development workflow is SDD, and the tool we are using now is OpenSpec + Claude UI+MiniMax2.5.
this prototyping will not involve multi-agents to do separate works and integrate. what we need to do is to define and iterate the sepc, which will give the AI agent instructions exactly what do do.[9:39 PM]is it right?
Mo Dong  [9:42 PM]
It's completely up to you what AI stack/tool chains you want to use here. Does not have to be OpenSpec  + claude toolchain + minimax model. All model costs/purchases will be reimbursed. If you need credit cards to get accesses to things like Cursor, Codex API, Claude or etc, let me know. I will purchase these access for you.
eason  [9:46 PM]
joined #vera-vibe. Also,  joined.Elliott  [10:21 PM]
能说中文不？我想问需求相关的问题：

C2PA的验证是在zkvm program里么？如果不是，我们怎么确保输入到pico zkvm program里的图片就是外部C2PA验证过的图片？
在zkvm里如果直接调用Photon代码方法做裁剪等工作，本身就是zkvm指令，因此这些代码指令天然就被包含在proof里。但是假如我们要提供一个界面来做裁剪等工作，然后保存了一张图片，后边要pico来证明界面上的操作，这个如何证明？只能是将图片操作步骤在pico program里重走一遍，然后输出图片的hash，和外部保存的图片hash比对？
[10:22 PM]如果有这个产品原型的UI flow就更清楚了
Ben Huang  [10:28 PM]
考虑这个是vibe-coding project，我用AI生成了一个答案哈。
architecture-qa.md 

# Brevis Vera — 核心架构 Q&A
​
## Q1: C2PA 的验证是在 ZKVM Program 里吗？如果不是，我们怎么确保输入到 Pico ZKVM Program 里的图片就是外部 C2PA 验证过的图片？
​
**答案：C2PA（或 mock ECDSA P-256）签名验证必须在 ZKVM 内部执行。**


Mo Dong  [10:29 PM]

是的 这个验证需要做在ZKVM里面
不管是不是UI上面的操作，最后的输出都是可以证明结果图片是由一系列操作从一个hash 为XXX，C2PA签名过的图片得来的。这个“一系列操作”可以认为是最后生成proof时候的public input
[10:32 PM]3. 我明确一下 本次tournament，限于我们ZKVM目前的本身实现不具备真的隐私属性，不要求真的隐私性，只是可以做成把具体的操作参数做成private input，表达这么一个一个意思
Elliott  [10:34 PM]
replied to a thread:这个AI回答靠谱啊
xiaozhou  [10:35 PM]
joined #vera-vibe.Mo Dong  [10:40 PM]
另外明确一个问题就是 product brief里面的内容是希望实现的基础功能，比如图片编辑类型只支持两个，但大家可以根据时间情况，在各方面进行自由发挥（比如更多功能），可以认为这是一个半命题作文
Alan  [10:45 PM]
joined #vera-vibe.Elliott  [11:53 PM]
刚才吃饭的时候我想了想，感觉C2PA可以不在zkvm里做。我说下我的看法哦。

我们的产品分为三个界面，第一个界面是图片编辑界面，用户编辑图片 picIn 后，生成图片 picOut 和记录编辑步骤保存格式等信息的 transformInfo 文件。第二个界面，就是一个 prove 界面，用户在这个界面上选择 picIn，picOut 和 transformInfo 文件，点击 zk prove（后台跑zkvm program 证明），生成 proof  以及 outputcommit - picIn 和 picOut 的 hash 值。第三个页面就是 verify 页面，可以验证 proof 和 picIn/picOut 的hash。然后picIn可以直接链接到C2PA证明的网页证明来源准确。通过hash验证文件的一致性 (edited) 
Mo Dong  [11:59 PM]
"然后picIn可以直接链接到C2PA证明的网页证明来源准确。" 这个如果在没有PicIn源文件的情况下，也可以通过某一个网站/service/cli tool的集成 证明这个metadata确实是被sign过的 并且certificate chain valid 也可以
Elliott  [11:59 PM]
没有picin源文件，我们zkvm无法重放编辑
Mo Dong  [12:00 AM]
验证者没有PicIn
[12:00 AM]prover肯定要有的哈
Elliott  [12:02 AM]
那就提供一个网站，可查询和记录 hash 和 metadata 的关联关系（甚至可以根据hash直接提供原图），只要有 picIn 的hash值就行 (edited) 
Mo Dong  [12:03 AM]
验证者不能看到Pic In
Elliott  [12:04 AM]
那就不用提供原图，只根据picIn hash看到sign的metadata (edited) 
Mo Dong  [12:04 AM]
这是要求 然后只要有valid验证的chain就行 我不是很清楚现在是不是有图片hash直接能够查到验证信息的网站服务
[12:04 AM]或者自己build一个也行
[12:04 AM]因为signature sign的也是metadata
[12:05 AM]这个要求是 能验真 确实能验就行
Elliott  [12:05 AM]
因为C2PA在zkvm里验证我的直觉是很耗算力，感觉没必要
Mo Dong  [12:06 AM]
只要chain of attestation是完整的 就可以 C2PA验证放在ZK里面我感觉可能的好处是可以把这个证明放到chain上面 end to end做验证
[12:07 AM]不强制要求 在目前半命题的tournament下 不强制要求on-chain verification (edited) 
Ben Huang  [12:37 AM]
https://github.com/benhuang2025/vera-vibe
AI已经给我生成一版了。我按照README试了一下，可以验证通过。我把图片稍微改了下之后，就verify失败了。Only visible to youGitHub  [12:37 AM]
To see rich preview of links from private repositories, connect to your GitHub account using /github signin. You can mute the rich previews by running /github settings.Ben Huang  [12:37 AM]
Screenshot 2026-03-02 at 12.10.34 AM.png [12:39 AM]我明儿再看下这个有不有啥bug，现在应该是支持JPEG和PNG
Mo Dong  [12:50 AM]
咱们继续自由发挥
[12:50 AM]半命题啊朋友们
[12:50 AM]英文界面谢谢
[12:50 AM]咱们周三产品发布的blog我都写好了
[12:50 AM]:joy:
Xiao Liu  [1:54 AM]
https://github.com/liuxiaobleach/pico-c2pa 按pico模版生成了可以verify c2pa的cli（目前没UI，运行按readme） (edited) 
Victor  [2:08 AM]
以及这个后面是要做到能production deployment，实际外部用户可以用的
Mo Dong  [2:11 AM]
对对对
[2:11 AM]咱们是真的要做这个产品的
Jie Tang  [3:12 AM]
replied to a thread:C2PA ECDSA P-256的verify需要x509-cert signer的public key， 测试图片的sign的public key可以提供下吗？给65byte的那个uncompressed public key就行 (edited) 
[3:12 AM]@Mo Dong
Mo Dong  [3:13 AM]
自己找找
[3:13 AM]Binary sigcert.p12Binary[3:13 AM]这个是设备的sig cert
[3:14 AM]default certificate
Jie Tang  [3:16 AM]
OK的
Mo Dong  [5:10 AM]
有反应说图片文件过大 我弄了个最小的形式
[5:10 AM]Zip DSC00056.JPG.zipZipElliott  [8:09 AM]
就算3M文件，在本机zkvm 里 hash也很慢啊。。。
[8:09 AM]kecaak还有precompile，sha256还没有
Mo Dong  [8:09 AM]
多慢
Elliott  [8:09 AM]
跑不出来，mock了
[8:09 AM]反正本机跑了好一会儿出不来
Mo Dong  [8:10 AM]
@Alan 有没有gpu server支持一下
Elliott  [8:10 AM]
demo还不需要吧。。。
Mo Dong  [8:10 AM]
end to end 尽量别mock
[8:11 AM]我们gpu应该有空余机器
Jie Tang  [8:11 AM]
我这里可以证
[8:11 AM]cpu应该可以啊
Elliott  [8:12 AM]
证明了什么内容呢
Jie Tang  [8:12 AM]
edit的prove
Elliott  [8:13 AM]
有哪些操作啊
[8:13 AM]program贴出来看下
Jie Tang  [8:15 AM]
待会 等我验证完
Ben Huang  [8:24 AM]
我这边暂时是用了降像素处理的，不然也是在单cpu上很慢。
在 mock-signer 签名阶段，并没有直接哈希全量原始像素，而是先将图片降采样至 256x256 (RGB8)，然后再进行 P-256 签名。
计算量对比：
3MB 原图：哈希开销极大，可能触发 OOM 或超长等待。
256x256 图片：数据量约 192KB。在 Pico 里的总周期约为 24 Million Cycles。这个量级在普通笔记本 CPU 上用 emulate 模式可以在秒级跑完，逻辑验证非常丝滑。
安全合规性建议：在实际 C2PA 行业标准中，相机硬件本身也经常对内容的低分辨率哈希 (Lower-res hash) 或 预览图断言 (Thumbnail Assertion) 进行签名。因此，在 ZK 证明中验证一个“采样后的签名图”，在逻辑完备性上与原图是一致的。Mo Dong  [8:27 AM]
如果原图太大了 用mock flow也行 做了完整flow的话 可以跟 @Alan 这里问问有没有gpu闲置机器可以用的
Ben Huang  [8:28 AM]
我可以用我这段时间做开发的gpu闲置机器试试哈 (edited) 
Alan  [8:46 AM]
gpu有的，@eason @Ben Huang 可以看一下哪个空闲
Alan  [8:47 AM]
这个会有这么大么？可以看一下有多少个cycles/chunks哈
Mo Dong  [8:54 AM]
我直观感觉 3M的hash gpu应该能顶一顶
Jie Tang  [9:16 AM]
感觉可能不是hash的问题，是不是ecdsa p256的身份验证是bottleneck？但是这里目前似乎只能用mock-signer来做，假设现在是sony设备签的，换成尼康或者其他根证书签的，不是又无法验证了，身份验证放到zkvm里面似乎不太好解决。 (edited) 
Jinyao  [9:24 AM]
另外感觉是我vibe coding提示不对(方向错误)，生成出来的code总是想把hash send给zkVM, 这种approach会存在security问题吗？
However, we don't actually need to send the full image to the zkVM. Instead, we use the hash from the C2PA metadata (the claim_hash field) as a commitment to the original image. This is a common pattern in ZK proofs - proving knowledge of something without revealing it.

Here's the current flow:

Prover has the full original image (9MB) and extracts C2PA metadata including the claim_hash
Prover applies image operations and creates the final image
Prover sends to zkVM:
claim_hash (32 bytes) - from C2PA metadata
Operations (small parameters like crop coordinates, resize dimensions)
Final image hash (32 bytes) - computed from the output file
zkVM computes a commitment: H(claim_hash || operations) and outputs this as public value
The zkVM doesn't need the full image - it only needs the hash commitment from C2PA. The security relies on the C2PA signature being valid (proving the original image was authentic), and the ZK proof proving that the operations were applied to produce the final image hash.Victor  [9:25 AM]
replied to a thread:可能需要一个ecdsa p256的precompile :joy:
Jie Tang  [9:28 AM]
之前加过这个patch: https://github.com/brevis-network/signatures curve得选取下p256
Victor  [9:30 AM]
replied to a thread:@Alan secp256r1 precompile我们支持吗？secp256k1我知道是work的
Jie Tang  [9:32 AM]
不过我刚才的意思是，这可能是个产品问题，我们现在vm里面内置一个public key来做签名的验证，但是其他设备签的图片就verify不了了。另外一个方案就是把public key作为public input从前端传入，verify的时候vm去获取来验证，但是但是UI就有点confused。

否则只能mock了 (edited) 
Mo Dong  [9:33 AM]
replied to a thread:尼康 索尼 根证书都可以认为是public input和public knowledge
Mo Dong  [9:36 AM]
replied to a thread:你说的没错，不同设备制造商的public key是一个public information，也要作为ZKVM的public input的一部分，你想，将来的验证flow就是，索尼的设备用索尼的根证书验证，尼康的用尼康的，莱卡用莱卡的，而这些根证书，都假设是所有人都知道并可以验证是他们的根证书的
Jie Tang  [9:37 AM]
ok 那就没问题了
Elliott  [9:44 AM]
我怎么还是感觉设备签名不需要在zkvm里验证啊？zkvm只需要证明一个文件经过几次操作变成另一个文件，然后output commit里有这两个文件的hash，通过一个权威网站可以查询原文件hash的有效性，然后编辑后的文件和后一个hash一致，不就可以了么？
Jie Tang  [9:45 AM]
权威网站也解决不了来源可信的问题呀
Elliott  [9:46 AM]
权威网站里就是验证原文件的签名啊，这个任何人有文件都可以验证，不需要zk proof额外再证一次
Jie Tang  [9:46 AM]
他们都中心化的验证
[9:47 AM]跟我们自己 host server 验证就没区别了 (edited) 
Elliott  [9:49 AM]
zk proof一样的道理啊，可以我们验证，也可以别人验证。我的意思是图片原文件本身就包含自证信息。我们zkvm所需要确定的一点是编辑的起点文件是来自于正确的文件，这个通过hash关联就可以
Mo Dong  [9:52 AM]
replied to a thread:我认为你说的可能是对的，但我没看具体的签名细节 所以不完全确定

我的理解是

相机签名，目前签名的是一个相片的metadata，这个metadata里面有啥，我没仔细研究过，那既然签名的是metadata，这个metadata里面是不是有hash，这个hash是不是和ZKVM的public input的hash是一样的。如果是一样的，那也不需要什么网站了，直接扔一个证书信任的key签出来的metadata就好了。验证者拿到的是 metadata，metadata的签名，照片编辑这个计算的zkproof。他要干两件事，1. 验证metadata签名X509 cert chain是对的，这个过程中要访问sony的根证书，然后验证；2. 验证计算是对的，这个靠ZK Proof。

如果确实是这样的话，在verifier不是blockchain的时候，我的理解是，这个第一步 是不是在VM里面做是无所谓的。

但是当verifier是一个blockchain smart contract的时候，你就需要把所有东西都包进去了，否则onchain也算不了那个sig check。
Elliott  [9:54 AM]
smart contract的话确实应该是都包含
Mo Dong  [9:55 AM]
你上午第一次问这个问题的时候，我的回答就是，命题作文的基础要求可以不假设需要smart contract验证，但是需要确定这个信任链条是work的
[9:57 AM]这个问题 不能假设 需要研究并且有研究的proof
Elliott  [9:57 AM]
我现在有另一个问题，我记得老董说用户验证的时候只需要提供编辑后的文件，以及被编辑的步骤。我们喂给zkvm private input的原文件是怎么找出来的？
Mo Dong  [9:57 AM]
没懂这个问题是啥
Elliott  [9:59 AM]
你想想啊，我们有个界面，用户去验证一个被编辑过后的文件的真实性，他需要提供最开始的那个设备输出的源文件么？ (edited) 
Mo Dong  [10:00 AM]
不需要源文件
[10:00 AM]建议研究zkvm的工作原理
[10:00 AM]可以问下AI
[10:00 AM]verfier需要啥 不需要啥
[10:01 AM]我可以说这不属于产品问题 产品需求是可以实现的，工程问题我就不回答了 (edited) 
Elliott  [10:06 AM]
哦，我知道了，我搞错了。是编辑者在编辑后就跑zkvm program生成了证明，这个proof被比如smart contract验证后，源文件和编辑后的文件的hash映射就保存在onchain，当然源文件的metadata也可以输出存在onchain(如有查询必要)。后边的verifier只需要直接提供编辑后的文件，我们算下这个文件hash，onchain存在valid的映射即可
Ryan Compton  [4:44 PM]
joined #vera-vibe.Ben Huang  [7:23 PM]
Previously on my side, the slow verification for the original image (not the 256×256 downscaled version) was due to the heavy emulator in Pico zkVM, which uses the old interpreter-based implementation as the default.
With a single CPU core, the emulator took about 4 minutes. After switching to the latest AOT-optimized emulator and enabling parallelism across 64 CPU cores, the emulator runtime was reduced to around 1 second.Ben Huang  [7:34 PM]
This is the performance of test runs by now
Screenshot 2026-03-02 at 7.34.00 PM.png Only visible to youGitHub  [7:40 PM]
To see rich preview of links from private repositories, connect to your GitHub account using /github signin. You can mute the rich previews by running /github settings.Victor  [8:51 PM]
replied to a thread:这个”latest AOT optimized emulator”是在我们的public release吗？ @Ben Huang 
Only visible to youGitHub  [8:55 PM]
To see rich preview of links from private repositories, connect to your GitHub account using /github signin. You can mute the rich previews by running /github settings.Mo Dong  [9:18 PM]
感觉大家做的速度都挺快的，我来发个匿名投票调查 看看我们需不需要到明天早上 还是今天下午sync直接present好了

https://forms.gle/i3q8XaGdFMTrXSw68
Google Docs进度调查https://forms.gle/i3q8XaGdFMTrXSw68Ryan Compton  [9:19 PM]
left #vera-vibe.Alan  [9:27 PM]
replied to a thread:有一个测试用例可以参考：https://github.com/brevis-network/pico/tree/main/examples/reth/aot/src
Mo Dong  [9:33 PM]
@Ben Huang 我强调一下哈 我们这个是内部tournament，注意不要把implementation细节和代码发出来
Mo Dong  [9:34 PM]
大家有什么产品的问题可以讨论 但是不要做代码共享
Mo Dong  [9:34 PM]
workflow之后present的时候我们再一起分享