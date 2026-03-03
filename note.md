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