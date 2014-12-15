/* crc32tables.rs -- tables for rapid CRC calculation
 * Generated automatically by crc32gen.rs
 */

pub static CRC_TABLE:[[u32, ..0x100], ..8] = [
  [
    0x00000000, 0x77073096, 0xee0e612c, 0x990951ba, 0x076dc419,
    0x706af48f, 0xe963a535, 0x9e6495a3, 0x0edb8832, 0x79dcb8a4,
    0xe0d5e91e, 0x97d2d988, 0x09b64c2b, 0x7eb17cbd, 0xe7b82d07,
    0x90bf1d91, 0x1db71064, 0x6ab020f2, 0xf3b97148, 0x84be41de,
    0x1adad47d, 0x6ddde4eb, 0xf4d4b551, 0x83d385c7, 0x136c9856,
    0x646ba8c0, 0xfd62f97a, 0x8a65c9ec, 0x14015c4f, 0x63066cd9,
    0xfa0f3d63, 0x8d080df5, 0x3b6e20c8, 0x4c69105e, 0xd56041e4,
    0xa2677172, 0x3c03e4d1, 0x4b04d447, 0xd20d85fd, 0xa50ab56b,
    0x35b5a8fa, 0x42b2986c, 0xdbbbc9d6, 0xacbcf940, 0x32d86ce3,
    0x45df5c75, 0xdcd60dcf, 0xabd13d59, 0x26d930ac, 0x51de003a,
    0xc8d75180, 0xbfd06116, 0x21b4f4b5, 0x56b3c423, 0xcfba9599,
    0xb8bda50f, 0x2802b89e, 0x5f058808, 0xc60cd9b2, 0xb10be924,
    0x2f6f7c87, 0x58684c11, 0xc1611dab, 0xb6662d3d, 0x76dc4190,
    0x01db7106, 0x98d220bc, 0xefd5102a, 0x71b18589, 0x06b6b51f,
    0x9fbfe4a5, 0xe8b8d433, 0x7807c9a2, 0x0f00f934, 0x9609a88e,
    0xe10e9818, 0x7f6a0dbb, 0x086d3d2d, 0x91646c97, 0xe6635c01,
    0x6b6b51f4, 0x1c6c6162, 0x856530d8, 0xf262004e, 0x6c0695ed,
    0x1b01a57b, 0x8208f4c1, 0xf50fc457, 0x65b0d9c6, 0x12b7e950,
    0x8bbeb8ea, 0xfcb9887c, 0x62dd1ddf, 0x15da2d49, 0x8cd37cf3,
    0xfbd44c65, 0x4db26158, 0x3ab551ce, 0xa3bc0074, 0xd4bb30e2,
    0x4adfa541, 0x3dd895d7, 0xa4d1c46d, 0xd3d6f4fb, 0x4369e96a,
    0x346ed9fc, 0xad678846, 0xda60b8d0, 0x44042d73, 0x33031de5,
    0xaa0a4c5f, 0xdd0d7cc9, 0x5005713c, 0x270241aa, 0xbe0b1010,
    0xc90c2086, 0x5768b525, 0x206f85b3, 0xb966d409, 0xce61e49f,
    0x5edef90e, 0x29d9c998, 0xb0d09822, 0xc7d7a8b4, 0x59b33d17,
    0x2eb40d81, 0xb7bd5c3b, 0xc0ba6cad, 0xedb88320, 0x9abfb3b6,
    0x03b6e20c, 0x74b1d29a, 0xead54739, 0x9dd277af, 0x04db2615,
    0x73dc1683, 0xe3630b12, 0x94643b84, 0x0d6d6a3e, 0x7a6a5aa8,
    0xe40ecf0b, 0x9309ff9d, 0x0a00ae27, 0x7d079eb1, 0xf00f9344,
    0x8708a3d2, 0x1e01f268, 0x6906c2fe, 0xf762575d, 0x806567cb,
    0x196c3671, 0x6e6b06e7, 0xfed41b76, 0x89d32be0, 0x10da7a5a,
    0x67dd4acc, 0xf9b9df6f, 0x8ebeeff9, 0x17b7be43, 0x60b08ed5,
    0xd6d6a3e8, 0xa1d1937e, 0x38d8c2c4, 0x4fdff252, 0xd1bb67f1,
    0xa6bc5767, 0x3fb506dd, 0x48b2364b, 0xd80d2bda, 0xaf0a1b4c,
    0x36034af6, 0x41047a60, 0xdf60efc3, 0xa867df55, 0x316e8eef,
    0x4669be79, 0xcb61b38c, 0xbc66831a, 0x256fd2a0, 0x5268e236,
    0xcc0c7795, 0xbb0b4703, 0x220216b9, 0x5505262f, 0xc5ba3bbe,
    0xb2bd0b28, 0x2bb45a92, 0x5cb36a04, 0xc2d7ffa7, 0xb5d0cf31,
    0x2cd99e8b, 0x5bdeae1d, 0x9b64c2b0, 0xec63f226, 0x756aa39c,
    0x026d930a, 0x9c0906a9, 0xeb0e363f, 0x72076785, 0x05005713,
    0x95bf4a82, 0xe2b87a14, 0x7bb12bae, 0x0cb61b38, 0x92d28e9b,
    0xe5d5be0d, 0x7cdcefb7, 0x0bdbdf21, 0x86d3d2d4, 0xf1d4e242,
    0x68ddb3f8, 0x1fda836e, 0x81be16cd, 0xf6b9265b, 0x6fb077e1,
    0x18b74777, 0x88085ae6, 0xff0f6a70, 0x66063bca, 0x11010b5c,
    0x8f659eff, 0xf862ae69, 0x616bffd3, 0x166ccf45, 0xa00ae278,
    0xd70dd2ee, 0x4e048354, 0x3903b3c2, 0xa7672661, 0xd06016f7,
    0x4969474d, 0x3e6e77db, 0xaed16a4a, 0xd9d65adc, 0x40df0b66,
    0x37d83bf0, 0xa9bcae53, 0xdebb9ec5, 0x47b2cf7f, 0x30b5ffe9,
    0xbdbdf21c, 0xcabac28a, 0x53b39330, 0x24b4a3a6, 0xbad03605,
    0xcdd70693, 0x54de5729, 0x23d967bf, 0xb3667a2e, 0xc4614ab8,
    0x5d681b02, 0x2a6f2b94, 0xb40bbe37, 0xc30c8ea1, 0x5a05df1b,
    0x2d02ef8d
// #ifdef BYFOUR
  ],
 [
    0x00000000, 0x191b3141, 0x32366282, 0x2b2d53c3, 0x646cc504,
    0x7d77f445, 0x565aa786, 0x4f4196c7, 0xc8d98a08, 0xd1c2bb49,
    0xfaefe88a, 0xe3f4d9cb, 0xacb54f0c, 0xb5ae7e4d, 0x9e832d8e,
    0x87981ccf, 0x4ac21251, 0x53d92310, 0x78f470d3, 0x61ef4192,
    0x2eaed755, 0x37b5e614, 0x1c98b5d7, 0x05838496, 0x821b9859,
    0x9b00a918, 0xb02dfadb, 0xa936cb9a, 0xe6775d5d, 0xff6c6c1c,
    0xd4413fdf, 0xcd5a0e9e, 0x958424a2, 0x8c9f15e3, 0xa7b24620,
    0xbea97761, 0xf1e8e1a6, 0xe8f3d0e7, 0xc3de8324, 0xdac5b265,
    0x5d5daeaa, 0x44469feb, 0x6f6bcc28, 0x7670fd69, 0x39316bae,
    0x202a5aef, 0x0b07092c, 0x121c386d, 0xdf4636f3, 0xc65d07b2,
    0xed705471, 0xf46b6530, 0xbb2af3f7, 0xa231c2b6, 0x891c9175,
    0x9007a034, 0x179fbcfb, 0x0e848dba, 0x25a9de79, 0x3cb2ef38,
    0x73f379ff, 0x6ae848be, 0x41c51b7d, 0x58de2a3c, 0xf0794f05,
    0xe9627e44, 0xc24f2d87, 0xdb541cc6, 0x94158a01, 0x8d0ebb40,
    0xa623e883, 0xbf38d9c2, 0x38a0c50d, 0x21bbf44c, 0x0a96a78f,
    0x138d96ce, 0x5ccc0009, 0x45d73148, 0x6efa628b, 0x77e153ca,
    0xbabb5d54, 0xa3a06c15, 0x888d3fd6, 0x91960e97, 0xded79850,
    0xc7cca911, 0xece1fad2, 0xf5facb93, 0x7262d75c, 0x6b79e61d,
    0x4054b5de, 0x594f849f, 0x160e1258, 0x0f152319, 0x243870da,
    0x3d23419b, 0x65fd6ba7, 0x7ce65ae6, 0x57cb0925, 0x4ed03864,
    0x0191aea3, 0x188a9fe2, 0x33a7cc21, 0x2abcfd60, 0xad24e1af,
    0xb43fd0ee, 0x9f12832d, 0x8609b26c, 0xc94824ab, 0xd05315ea,
    0xfb7e4629, 0xe2657768, 0x2f3f79f6, 0x362448b7, 0x1d091b74,
    0x04122a35, 0x4b53bcf2, 0x52488db3, 0x7965de70, 0x607eef31,
    0xe7e6f3fe, 0xfefdc2bf, 0xd5d0917c, 0xcccba03d, 0x838a36fa,
    0x9a9107bb, 0xb1bc5478, 0xa8a76539, 0x3b83984b, 0x2298a90a,
    0x09b5fac9, 0x10aecb88, 0x5fef5d4f, 0x46f46c0e, 0x6dd93fcd,
    0x74c20e8c, 0xf35a1243, 0xea412302, 0xc16c70c1, 0xd8774180,
    0x9736d747, 0x8e2de606, 0xa500b5c5, 0xbc1b8484, 0x71418a1a,
    0x685abb5b, 0x4377e898, 0x5a6cd9d9, 0x152d4f1e, 0x0c367e5f,
    0x271b2d9c, 0x3e001cdd, 0xb9980012, 0xa0833153, 0x8bae6290,
    0x92b553d1, 0xddf4c516, 0xc4eff457, 0xefc2a794, 0xf6d996d5,
    0xae07bce9, 0xb71c8da8, 0x9c31de6b, 0x852aef2a, 0xca6b79ed,
    0xd37048ac, 0xf85d1b6f, 0xe1462a2e, 0x66de36e1, 0x7fc507a0,
    0x54e85463, 0x4df36522, 0x02b2f3e5, 0x1ba9c2a4, 0x30849167,
    0x299fa026, 0xe4c5aeb8, 0xfdde9ff9, 0xd6f3cc3a, 0xcfe8fd7b,
    0x80a96bbc, 0x99b25afd, 0xb29f093e, 0xab84387f, 0x2c1c24b0,
    0x350715f1, 0x1e2a4632, 0x07317773, 0x4870e1b4, 0x516bd0f5,
    0x7a468336, 0x635db277, 0xcbfad74e, 0xd2e1e60f, 0xf9ccb5cc,
    0xe0d7848d, 0xaf96124a, 0xb68d230b, 0x9da070c8, 0x84bb4189,
    0x03235d46, 0x1a386c07, 0x31153fc4, 0x280e0e85, 0x674f9842,
    0x7e54a903, 0x5579fac0, 0x4c62cb81, 0x8138c51f, 0x9823f45e,
    0xb30ea79d, 0xaa1596dc, 0xe554001b, 0xfc4f315a, 0xd7626299,
    0xce7953d8, 0x49e14f17, 0x50fa7e56, 0x7bd72d95, 0x62cc1cd4,
    0x2d8d8a13, 0x3496bb52, 0x1fbbe891, 0x06a0d9d0, 0x5e7ef3ec,
    0x4765c2ad, 0x6c48916e, 0x7553a02f, 0x3a1236e8, 0x230907a9,
    0x0824546a, 0x113f652b, 0x96a779e4, 0x8fbc48a5, 0xa4911b66,
    0xbd8a2a27, 0xf2cbbce0, 0xebd08da1, 0xc0fdde62, 0xd9e6ef23,
    0x14bce1bd, 0x0da7d0fc, 0x268a833f, 0x3f91b27e, 0x70d024b9,
    0x69cb15f8, 0x42e6463b, 0x5bfd777a, 0xdc656bb5, 0xc57e5af4,
    0xee530937, 0xf7483876, 0xb809aeb1, 0xa1129ff0, 0x8a3fcc33,
    0x9324fd72
  ],
 [
    0x00000000, 0x01c26a37, 0x0384d46e, 0x0246be59, 0x0709a8dc,
    0x06cbc2eb, 0x048d7cb2, 0x054f1685, 0x0e1351b8, 0x0fd13b8f,
    0x0d9785d6, 0x0c55efe1, 0x091af964, 0x08d89353, 0x0a9e2d0a,
    0x0b5c473d, 0x1c26a370, 0x1de4c947, 0x1fa2771e, 0x1e601d29,
    0x1b2f0bac, 0x1aed619b, 0x18abdfc2, 0x1969b5f5, 0x1235f2c8,
    0x13f798ff, 0x11b126a6, 0x10734c91, 0x153c5a14, 0x14fe3023,
    0x16b88e7a, 0x177ae44d, 0x384d46e0, 0x398f2cd7, 0x3bc9928e,
    0x3a0bf8b9, 0x3f44ee3c, 0x3e86840b, 0x3cc03a52, 0x3d025065,
    0x365e1758, 0x379c7d6f, 0x35dac336, 0x3418a901, 0x3157bf84,
    0x3095d5b3, 0x32d36bea, 0x331101dd, 0x246be590, 0x25a98fa7,
    0x27ef31fe, 0x262d5bc9, 0x23624d4c, 0x22a0277b, 0x20e69922,
    0x2124f315, 0x2a78b428, 0x2bbade1f, 0x29fc6046, 0x283e0a71,
    0x2d711cf4, 0x2cb376c3, 0x2ef5c89a, 0x2f37a2ad, 0x709a8dc0,
    0x7158e7f7, 0x731e59ae, 0x72dc3399, 0x7793251c, 0x76514f2b,
    0x7417f172, 0x75d59b45, 0x7e89dc78, 0x7f4bb64f, 0x7d0d0816,
    0x7ccf6221, 0x798074a4, 0x78421e93, 0x7a04a0ca, 0x7bc6cafd,
    0x6cbc2eb0, 0x6d7e4487, 0x6f38fade, 0x6efa90e9, 0x6bb5866c,
    0x6a77ec5b, 0x68315202, 0x69f33835, 0x62af7f08, 0x636d153f,
    0x612bab66, 0x60e9c151, 0x65a6d7d4, 0x6464bde3, 0x662203ba,
    0x67e0698d, 0x48d7cb20, 0x4915a117, 0x4b531f4e, 0x4a917579,
    0x4fde63fc, 0x4e1c09cb, 0x4c5ab792, 0x4d98dda5, 0x46c49a98,
    0x4706f0af, 0x45404ef6, 0x448224c1, 0x41cd3244, 0x400f5873,
    0x4249e62a, 0x438b8c1d, 0x54f16850, 0x55330267, 0x5775bc3e,
    0x56b7d609, 0x53f8c08c, 0x523aaabb, 0x507c14e2, 0x51be7ed5,
    0x5ae239e8, 0x5b2053df, 0x5966ed86, 0x58a487b1, 0x5deb9134,
    0x5c29fb03, 0x5e6f455a, 0x5fad2f6d, 0xe1351b80, 0xe0f771b7,
    0xe2b1cfee, 0xe373a5d9, 0xe63cb35c, 0xe7fed96b, 0xe5b86732,
    0xe47a0d05, 0xef264a38, 0xeee4200f, 0xeca29e56, 0xed60f461,
    0xe82fe2e4, 0xe9ed88d3, 0xebab368a, 0xea695cbd, 0xfd13b8f0,
    0xfcd1d2c7, 0xfe976c9e, 0xff5506a9, 0xfa1a102c, 0xfbd87a1b,
    0xf99ec442, 0xf85cae75, 0xf300e948, 0xf2c2837f, 0xf0843d26,
    0xf1465711, 0xf4094194, 0xf5cb2ba3, 0xf78d95fa, 0xf64fffcd,
    0xd9785d60, 0xd8ba3757, 0xdafc890e, 0xdb3ee339, 0xde71f5bc,
    0xdfb39f8b, 0xddf521d2, 0xdc374be5, 0xd76b0cd8, 0xd6a966ef,
    0xd4efd8b6, 0xd52db281, 0xd062a404, 0xd1a0ce33, 0xd3e6706a,
    0xd2241a5d, 0xc55efe10, 0xc49c9427, 0xc6da2a7e, 0xc7184049,
    0xc25756cc, 0xc3953cfb, 0xc1d382a2, 0xc011e895, 0xcb4dafa8,
    0xca8fc59f, 0xc8c97bc6, 0xc90b11f1, 0xcc440774, 0xcd866d43,
    0xcfc0d31a, 0xce02b92d, 0x91af9640, 0x906dfc77, 0x922b422e,
    0x93e92819, 0x96a63e9c, 0x976454ab, 0x9522eaf2, 0x94e080c5,
    0x9fbcc7f8, 0x9e7eadcf, 0x9c381396, 0x9dfa79a1, 0x98b56f24,
    0x99770513, 0x9b31bb4a, 0x9af3d17d, 0x8d893530, 0x8c4b5f07,
    0x8e0de15e, 0x8fcf8b69, 0x8a809dec, 0x8b42f7db, 0x89044982,
    0x88c623b5, 0x839a6488, 0x82580ebf, 0x801eb0e6, 0x81dcdad1,
    0x8493cc54, 0x8551a663, 0x8717183a, 0x86d5720d, 0xa9e2d0a0,
    0xa820ba97, 0xaa6604ce, 0xaba46ef9, 0xaeeb787c, 0xaf29124b,
    0xad6fac12, 0xacadc625, 0xa7f18118, 0xa633eb2f, 0xa4755576,
    0xa5b73f41, 0xa0f829c4, 0xa13a43f3, 0xa37cfdaa, 0xa2be979d,
    0xb5c473d0, 0xb40619e7, 0xb640a7be, 0xb782cd89, 0xb2cddb0c,
    0xb30fb13b, 0xb1490f62, 0xb08b6555, 0xbbd72268, 0xba15485f,
    0xb853f606, 0xb9919c31, 0xbcde8ab4, 0xbd1ce083, 0xbf5a5eda,
    0xbe9834ed
  ],
 [
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000
  ],
 [
    0x00000000, 0x96300777, 0x2c610eee, 0xba510999, 0x19c46d07,
    0x8ff46a70, 0x35a563e9, 0xa395649e, 0x3288db0e, 0xa4b8dc79,
    0x1ee9d5e0, 0x88d9d297, 0x2b4cb609, 0xbd7cb17e, 0x072db8e7,
    0x911dbf90, 0x6410b71d, 0xf220b06a, 0x4871b9f3, 0xde41be84,
    0x7dd4da1a, 0xebe4dd6d, 0x51b5d4f4, 0xc785d383, 0x56986c13,
    0xc0a86b64, 0x7af962fd, 0xecc9658a, 0x4f5c0114, 0xd96c0663,
    0x633d0ffa, 0xf50d088d, 0xc8206e3b, 0x5e10694c, 0xe44160d5,
    0x727167a2, 0xd1e4033c, 0x47d4044b, 0xfd850dd2, 0x6bb50aa5,
    0xfaa8b535, 0x6c98b242, 0xd6c9bbdb, 0x40f9bcac, 0xe36cd832,
    0x755cdf45, 0xcf0dd6dc, 0x593dd1ab, 0xac30d926, 0x3a00de51,
    0x8051d7c8, 0x1661d0bf, 0xb5f4b421, 0x23c4b356, 0x9995bacf,
    0x0fa5bdb8, 0x9eb80228, 0x0888055f, 0xb2d90cc6, 0x24e90bb1,
    0x877c6f2f, 0x114c6858, 0xab1d61c1, 0x3d2d66b6, 0x9041dc76,
    0x0671db01, 0xbc20d298, 0x2a10d5ef, 0x8985b171, 0x1fb5b606,
    0xa5e4bf9f, 0x33d4b8e8, 0xa2c90778, 0x34f9000f, 0x8ea80996,
    0x18980ee1, 0xbb0d6a7f, 0x2d3d6d08, 0x976c6491, 0x015c63e6,
    0xf4516b6b, 0x62616c1c, 0xd8306585, 0x4e0062f2, 0xed95066c,
    0x7ba5011b, 0xc1f40882, 0x57c40ff5, 0xc6d9b065, 0x50e9b712,
    0xeab8be8b, 0x7c88b9fc, 0xdf1ddd62, 0x492dda15, 0xf37cd38c,
    0x654cd4fb, 0x5861b24d, 0xce51b53a, 0x7400bca3, 0xe230bbd4,
    0x41a5df4a, 0xd795d83d, 0x6dc4d1a4, 0xfbf4d6d3, 0x6ae96943,
    0xfcd96e34, 0x468867ad, 0xd0b860da, 0x732d0444, 0xe51d0333,
    0x5f4c0aaa, 0xc97c0ddd, 0x3c710550, 0xaa410227, 0x10100bbe,
    0x86200cc9, 0x25b56857, 0xb3856f20, 0x09d466b9, 0x9fe461ce,
    0x0ef9de5e, 0x98c9d929, 0x2298d0b0, 0xb4a8d7c7, 0x173db359,
    0x810db42e, 0x3b5cbdb7, 0xad6cbac0, 0x2083b8ed, 0xb6b3bf9a,
    0x0ce2b603, 0x9ad2b174, 0x3947d5ea, 0xaf77d29d, 0x1526db04,
    0x8316dc73, 0x120b63e3, 0x843b6494, 0x3e6a6d0d, 0xa85a6a7a,
    0x0bcf0ee4, 0x9dff0993, 0x27ae000a, 0xb19e077d, 0x44930ff0,
    0xd2a30887, 0x68f2011e, 0xfec20669, 0x5d5762f7, 0xcb676580,
    0x71366c19, 0xe7066b6e, 0x761bd4fe, 0xe02bd389, 0x5a7ada10,
    0xcc4add67, 0x6fdfb9f9, 0xf9efbe8e, 0x43beb717, 0xd58eb060,
    0xe8a3d6d6, 0x7e93d1a1, 0xc4c2d838, 0x52f2df4f, 0xf167bbd1,
    0x6757bca6, 0xdd06b53f, 0x4b36b248, 0xda2b0dd8, 0x4c1b0aaf,
    0xf64a0336, 0x607a0441, 0xc3ef60df, 0x55df67a8, 0xef8e6e31,
    0x79be6946, 0x8cb361cb, 0x1a8366bc, 0xa0d26f25, 0x36e26852,
    0x95770ccc, 0x03470bbb, 0xb9160222, 0x2f260555, 0xbe3bbac5,
    0x280bbdb2, 0x925ab42b, 0x046ab35c, 0xa7ffd7c2, 0x31cfd0b5,
    0x8b9ed92c, 0x1daede5b, 0xb0c2649b, 0x26f263ec, 0x9ca36a75,
    0x0a936d02, 0xa906099c, 0x3f360eeb, 0x85670772, 0x13570005,
    0x824abf95, 0x147ab8e2, 0xae2bb17b, 0x381bb60c, 0x9b8ed292,
    0x0dbed5e5, 0xb7efdc7c, 0x21dfdb0b, 0xd4d2d386, 0x42e2d4f1,
    0xf8b3dd68, 0x6e83da1f, 0xcd16be81, 0x5b26b9f6, 0xe177b06f,
    0x7747b718, 0xe65a0888, 0x706a0fff, 0xca3b0666, 0x5c0b0111,
    0xff9e658f, 0x69ae62f8, 0xd3ff6b61, 0x45cf6c16, 0x78e20aa0,
    0xeed20dd7, 0x5483044e, 0xc2b30339, 0x612667a7, 0xf71660d0,
    0x4d476949, 0xdb776e3e, 0x4a6ad1ae, 0xdc5ad6d9, 0x660bdf40,
    0xf03bd837, 0x53aebca9, 0xc59ebbde, 0x7fcfb247, 0xe9ffb530,
    0x1cf2bdbd, 0x8ac2baca, 0x3093b353, 0xa6a3b424, 0x0536d0ba,
    0x9306d7cd, 0x2957de54, 0xbf67d923, 0x2e7a66b3, 0xb84a61c4,
    0x021b685d, 0x942b6f2a, 0x37be0bb4, 0xa18e0cc3, 0x1bdf055a,
    0x8def022d
  ],
 [
    0x00000000, 0x41311b19, 0x82623632, 0xc3532d2b, 0x04c56c64,
    0x45f4777d, 0x86a75a56, 0xc796414f, 0x088ad9c8, 0x49bbc2d1,
    0x8ae8effa, 0xcbd9f4e3, 0x0c4fb5ac, 0x4d7eaeb5, 0x8e2d839e,
    0xcf1c9887, 0x5112c24a, 0x1023d953, 0xd370f478, 0x9241ef61,
    0x55d7ae2e, 0x14e6b537, 0xd7b5981c, 0x96848305, 0x59981b82,
    0x18a9009b, 0xdbfa2db0, 0x9acb36a9, 0x5d5d77e6, 0x1c6c6cff,
    0xdf3f41d4, 0x9e0e5acd, 0xa2248495, 0xe3159f8c, 0x2046b2a7,
    0x6177a9be, 0xa6e1e8f1, 0xe7d0f3e8, 0x2483dec3, 0x65b2c5da,
    0xaaae5d5d, 0xeb9f4644, 0x28cc6b6f, 0x69fd7076, 0xae6b3139,
    0xef5a2a20, 0x2c09070b, 0x6d381c12, 0xf33646df, 0xb2075dc6,
    0x715470ed, 0x30656bf4, 0xf7f32abb, 0xb6c231a2, 0x75911c89,
    0x34a00790, 0xfbbc9f17, 0xba8d840e, 0x79dea925, 0x38efb23c,
    0xff79f373, 0xbe48e86a, 0x7d1bc541, 0x3c2ade58, 0x054f79f0,
    0x447e62e9, 0x872d4fc2, 0xc61c54db, 0x018a1594, 0x40bb0e8d,
    0x83e823a6, 0xc2d938bf, 0x0dc5a038, 0x4cf4bb21, 0x8fa7960a,
    0xce968d13, 0x0900cc5c, 0x4831d745, 0x8b62fa6e, 0xca53e177,
    0x545dbbba, 0x156ca0a3, 0xd63f8d88, 0x970e9691, 0x5098d7de,
    0x11a9ccc7, 0xd2fae1ec, 0x93cbfaf5, 0x5cd76272, 0x1de6796b,
    0xdeb55440, 0x9f844f59, 0x58120e16, 0x1923150f, 0xda703824,
    0x9b41233d, 0xa76bfd65, 0xe65ae67c, 0x2509cb57, 0x6438d04e,
    0xa3ae9101, 0xe29f8a18, 0x21cca733, 0x60fdbc2a, 0xafe124ad,
    0xeed03fb4, 0x2d83129f, 0x6cb20986, 0xab2448c9, 0xea1553d0,
    0x29467efb, 0x687765e2, 0xf6793f2f, 0xb7482436, 0x741b091d,
    0x352a1204, 0xf2bc534b, 0xb38d4852, 0x70de6579, 0x31ef7e60,
    0xfef3e6e7, 0xbfc2fdfe, 0x7c91d0d5, 0x3da0cbcc, 0xfa368a83,
    0xbb07919a, 0x7854bcb1, 0x3965a7a8, 0x4b98833b, 0x0aa99822,
    0xc9fab509, 0x88cbae10, 0x4f5def5f, 0x0e6cf446, 0xcd3fd96d,
    0x8c0ec274, 0x43125af3, 0x022341ea, 0xc1706cc1, 0x804177d8,
    0x47d73697, 0x06e62d8e, 0xc5b500a5, 0x84841bbc, 0x1a8a4171,
    0x5bbb5a68, 0x98e87743, 0xd9d96c5a, 0x1e4f2d15, 0x5f7e360c,
    0x9c2d1b27, 0xdd1c003e, 0x120098b9, 0x533183a0, 0x9062ae8b,
    0xd153b592, 0x16c5f4dd, 0x57f4efc4, 0x94a7c2ef, 0xd596d9f6,
    0xe9bc07ae, 0xa88d1cb7, 0x6bde319c, 0x2aef2a85, 0xed796bca,
    0xac4870d3, 0x6f1b5df8, 0x2e2a46e1, 0xe136de66, 0xa007c57f,
    0x6354e854, 0x2265f34d, 0xe5f3b202, 0xa4c2a91b, 0x67918430,
    0x26a09f29, 0xb8aec5e4, 0xf99fdefd, 0x3accf3d6, 0x7bfde8cf,
    0xbc6ba980, 0xfd5ab299, 0x3e099fb2, 0x7f3884ab, 0xb0241c2c,
    0xf1150735, 0x32462a1e, 0x73773107, 0xb4e17048, 0xf5d06b51,
    0x3683467a, 0x77b25d63, 0x4ed7facb, 0x0fe6e1d2, 0xccb5ccf9,
    0x8d84d7e0, 0x4a1296af, 0x0b238db6, 0xc870a09d, 0x8941bb84,
    0x465d2303, 0x076c381a, 0xc43f1531, 0x850e0e28, 0x42984f67,
    0x03a9547e, 0xc0fa7955, 0x81cb624c, 0x1fc53881, 0x5ef42398,
    0x9da70eb3, 0xdc9615aa, 0x1b0054e5, 0x5a314ffc, 0x996262d7,
    0xd85379ce, 0x174fe149, 0x567efa50, 0x952dd77b, 0xd41ccc62,
    0x138a8d2d, 0x52bb9634, 0x91e8bb1f, 0xd0d9a006, 0xecf37e5e,
    0xadc26547, 0x6e91486c, 0x2fa05375, 0xe836123a, 0xa9070923,
    0x6a542408, 0x2b653f11, 0xe479a796, 0xa548bc8f, 0x661b91a4,
    0x272a8abd, 0xe0bccbf2, 0xa18dd0eb, 0x62defdc0, 0x23efe6d9,
    0xbde1bc14, 0xfcd0a70d, 0x3f838a26, 0x7eb2913f, 0xb924d070,
    0xf815cb69, 0x3b46e642, 0x7a77fd5b, 0xb56b65dc, 0xf45a7ec5,
    0x370953ee, 0x763848f7, 0xb1ae09b8, 0xf09f12a1, 0x33cc3f8a,
    0x72fd2493
  ],
 [
    0x00000000, 0x376ac201, 0x6ed48403, 0x59be4602, 0xdca80907,
    0xebc2cb06, 0xb27c8d04, 0x85164f05, 0xb851130e, 0x8f3bd10f,
    0xd685970d, 0xe1ef550c, 0x64f91a09, 0x5393d808, 0x0a2d9e0a,
    0x3d475c0b, 0x70a3261c, 0x47c9e41d, 0x1e77a21f, 0x291d601e,
    0xac0b2f1b, 0x9b61ed1a, 0xc2dfab18, 0xf5b56919, 0xc8f23512,
    0xff98f713, 0xa626b111, 0x914c7310, 0x145a3c15, 0x2330fe14,
    0x7a8eb816, 0x4de47a17, 0xe0464d38, 0xd72c8f39, 0x8e92c93b,
    0xb9f80b3a, 0x3cee443f, 0x0b84863e, 0x523ac03c, 0x6550023d,
    0x58175e36, 0x6f7d9c37, 0x36c3da35, 0x01a91834, 0x84bf5731,
    0xb3d59530, 0xea6bd332, 0xdd011133, 0x90e56b24, 0xa78fa925,
    0xfe31ef27, 0xc95b2d26, 0x4c4d6223, 0x7b27a022, 0x2299e620,
    0x15f32421, 0x28b4782a, 0x1fdeba2b, 0x4660fc29, 0x710a3e28,
    0xf41c712d, 0xc376b32c, 0x9ac8f52e, 0xada2372f, 0xc08d9a70,
    0xf7e75871, 0xae591e73, 0x9933dc72, 0x1c259377, 0x2b4f5176,
    0x72f11774, 0x459bd575, 0x78dc897e, 0x4fb64b7f, 0x16080d7d,
    0x2162cf7c, 0xa4748079, 0x931e4278, 0xcaa0047a, 0xfdcac67b,
    0xb02ebc6c, 0x87447e6d, 0xdefa386f, 0xe990fa6e, 0x6c86b56b,
    0x5bec776a, 0x02523168, 0x3538f369, 0x087faf62, 0x3f156d63,
    0x66ab2b61, 0x51c1e960, 0xd4d7a665, 0xe3bd6464, 0xba032266,
    0x8d69e067, 0x20cbd748, 0x17a11549, 0x4e1f534b, 0x7975914a,
    0xfc63de4f, 0xcb091c4e, 0x92b75a4c, 0xa5dd984d, 0x989ac446,
    0xaff00647, 0xf64e4045, 0xc1248244, 0x4432cd41, 0x73580f40,
    0x2ae64942, 0x1d8c8b43, 0x5068f154, 0x67023355, 0x3ebc7557,
    0x09d6b756, 0x8cc0f853, 0xbbaa3a52, 0xe2147c50, 0xd57ebe51,
    0xe839e25a, 0xdf53205b, 0x86ed6659, 0xb187a458, 0x3491eb5d,
    0x03fb295c, 0x5a456f5e, 0x6d2fad5f, 0x801b35e1, 0xb771f7e0,
    0xeecfb1e2, 0xd9a573e3, 0x5cb33ce6, 0x6bd9fee7, 0x3267b8e5,
    0x050d7ae4, 0x384a26ef, 0x0f20e4ee, 0x569ea2ec, 0x61f460ed,
    0xe4e22fe8, 0xd388ede9, 0x8a36abeb, 0xbd5c69ea, 0xf0b813fd,
    0xc7d2d1fc, 0x9e6c97fe, 0xa90655ff, 0x2c101afa, 0x1b7ad8fb,
    0x42c49ef9, 0x75ae5cf8, 0x48e900f3, 0x7f83c2f2, 0x263d84f0,
    0x115746f1, 0x944109f4, 0xa32bcbf5, 0xfa958df7, 0xcdff4ff6,
    0x605d78d9, 0x5737bad8, 0x0e89fcda, 0x39e33edb, 0xbcf571de,
    0x8b9fb3df, 0xd221f5dd, 0xe54b37dc, 0xd80c6bd7, 0xef66a9d6,
    0xb6d8efd4, 0x81b22dd5, 0x04a462d0, 0x33cea0d1, 0x6a70e6d3,
    0x5d1a24d2, 0x10fe5ec5, 0x27949cc4, 0x7e2adac6, 0x494018c7,
    0xcc5657c2, 0xfb3c95c3, 0xa282d3c1, 0x95e811c0, 0xa8af4dcb,
    0x9fc58fca, 0xc67bc9c8, 0xf1110bc9, 0x740744cc, 0x436d86cd,
    0x1ad3c0cf, 0x2db902ce, 0x4096af91, 0x77fc6d90, 0x2e422b92,
    0x1928e993, 0x9c3ea696, 0xab546497, 0xf2ea2295, 0xc580e094,
    0xf8c7bc9f, 0xcfad7e9e, 0x9613389c, 0xa179fa9d, 0x246fb598,
    0x13057799, 0x4abb319b, 0x7dd1f39a, 0x3035898d, 0x075f4b8c,
    0x5ee10d8e, 0x698bcf8f, 0xec9d808a, 0xdbf7428b, 0x82490489,
    0xb523c688, 0x88649a83, 0xbf0e5882, 0xe6b01e80, 0xd1dadc81,
    0x54cc9384, 0x63a65185, 0x3a181787, 0x0d72d586, 0xa0d0e2a9,
    0x97ba20a8, 0xce0466aa, 0xf96ea4ab, 0x7c78ebae, 0x4b1229af,
    0x12ac6fad, 0x25c6adac, 0x1881f1a7, 0x2feb33a6, 0x765575a4,
    0x413fb7a5, 0xc429f8a0, 0xf3433aa1, 0xaafd7ca3, 0x9d97bea2,
    0xd073c4b5, 0xe71906b4, 0xbea740b6, 0x89cd82b7, 0x0cdbcdb2,
    0x3bb10fb3, 0x620f49b1, 0x55658bb0, 0x6822d7bb, 0x5f4815ba,
    0x06f653b8, 0x319c91b9, 0xb48adebc, 0x83e01cbd, 0xda5e5abf,
    0xed3498be
  ],
 [
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000, 0x00000000, 0x00000000, 0x00000000, 0x00000000,
    0x00000000
// #endif
  ]
];
