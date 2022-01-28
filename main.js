const { tlvExtractor } = require('.')

;(function main() {
  const tagSize = 4
  const lengthSize = 2
  const tlv = "!A0 04fa2 !B0 37af3cf6e9-7ddb-4e63-ba88-0477558f1d14 !C00369732ea9b-0b8b-4c48-997a-bfa1b05a2838"

  const tv = tlvExtractor(tlv, tagSize, lengthSize)
  
  console.log(tv)
})()