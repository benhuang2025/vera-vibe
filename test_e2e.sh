#!/bin/bash
set -e

# Colors for output
GREEN='\033[0;32m'
RED='\033[0;31m'
NC='\033[0m' # No Color

echo -e "${GREEN}🚀 Starting Brevis Vera End-to-End Test Suite${NC}\n"

# 1. Setup - Keygen
echo "--- 1. Keygen ---"
cd brevis-vera-zk
cargo run --bin mock-signer -- keygen --output private_key_test.pem > keygen_test.txt
PUBKEY=$(grep "Public Key (HEX):" keygen_test.txt | cut -d' ' -f4)
PUBKEY_HASH=$(echo -n $PUBKEY | xxd -r -p | openssl dgst -sha256 | cut -d' ' -f2)
echo "Generated Trusted PubKey Hash: $PUBKEY_HASH"

# 2. Capture - Sign Image
echo -e "\n--- 2. Capture & Sign ---"
# 💡 Change the SOURCE_IMAGE variable below to use your own photo from the images/ folder
SOURCE_IMAGE="../images/DSC00050.JPG"
cargo run --bin mock-signer -- sign --image "$SOURCE_IMAGE" --key private_key_test.pem --output ../signed_test.json

# 3. Edit
echo -e "\n--- 3. Edit ---"
cargo run --bin mock-signer -- edit --input ../signed_test.json --ops "crop:0,0,100,100;brightness:30" --output ../edited_test.png --manifest ../edit_manifest_test.json

# 4. Prove
echo -e "\n--- 4. ZK Proof (Emulation) ---"
cd prover
cargo run --release -- --photo ../../signed_test.json --manifest ../../edit_manifest_test.json --output ../../proof_package_test.json --edited-image ../../edited_test.png
cd ..

# 5. Verify - Case A: Valid
echo -e "\n--- 5A. Positive Case: Valid Image & Correct Key ---"
if cargo run --bin mock-signer -- verify --package ../proof_package_test.json --image ../edited_test.png --trusted-pubkey-hash $PUBKEY_HASH | grep "VERIFICATION SUCCESSFUL"; then
    echo -e "${GREEN}✅ Positive test passed!${NC}"
else
    echo -e "${RED}❌ Positive test failed!${NC}"
    exit 1
fi

# 6. Verify - Case B: Tamper Image
echo -e "\n--- 5B. Negative Case: Tampered Image (Brightness 40) ---"
# Create a slightly different image
cargo run --bin mock-signer -- edit --input ../signed_test.json --ops "crop:0,0,100,100;brightness:40" --output ../tampered_image.png --manifest ../dummy.json
if cargo run --bin mock-signer -- verify --package ../proof_package_test.json --image ../tampered_image.png --trusted-pubkey-hash $PUBKEY_HASH | grep "Image Hash MISMATCH"; then
    echo -e "${GREEN}✅ Negative test (tampered image) passed!${NC}"
else
    echo -e "${RED}❌ Tampered image was not detected!${NC}"
    exit 1
fi

# 7. Verify - Case C: Wrong Public Key
echo -e "\n--- 5C. Negative Case: Wrong Trusted Key ---"
WRONG_PUBKEY_HASH="0000000000000000000000000000000000000000000000000000000000000000"
if cargo run --bin mock-signer -- verify --package ../proof_package_test.json --image ../edited_test.png --trusted-pubkey-hash $WRONG_PUBKEY_HASH | grep "Signer is UNTRUSTED"; then
    echo -e "${GREEN}✅ Negative test (wrong pubkey) passed!${NC}"
else
    echo -e "${RED}❌ Wrong public key was not detected!${NC}"
    exit 1
fi

echo -e "\n${GREEN}🏆 ALL TESTS PASSED SUCCESSFULLY!${NC}"
