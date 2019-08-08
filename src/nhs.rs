/*
Licensed to the Apache Software Foundation (ASF) under one
or more contributor license agreements.  See the NOTICE file
distributed with this work for additional information
regarding copyright ownership.  The ASF licenses this file
to you under the Apache License, Version 2.0 (the
"License"); you may not use this file except in compliance
with the License.  You may obtain a copy of the License at

  http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing,
software distributed under the License is distributed on an
"AS IS" BASIS, WITHOUT WARRANTIES OR CONDITIONS OF ANY
KIND, either express or implied.  See the License for the
specific language governing permissions and limitations
under the License.
*/

/* NewHope Simple API high-level functions  */

use crate::rand::RAND;
use crate::sha3;
use crate::sha3::SHA3;

const PRIME: i32 = 0x3001; // q in Hex
const LGN: usize = 10; // Degree n=2^LGN
const ND: u32 = 0xF7002FFF; // 1/(R-q) mod R
const ONE: i32 = 0x2AC8; // R mod q
const R2MODP: u64 = 0x1620; // R^2 mod q

const DEGREE: usize = (1 << LGN);
const WL: usize = 32;

const INV: i32 = 0xeab;
const INVPR: i32 = 0x2c2a;

const ROOTS: [i32; 1024] = [
    0x2ac8, 0x2baf, 0x299b, 0x685, 0x2f04, 0x158d, 0x2d49, 0x24b5, 0x1edc, 0xab3, 0x2a95, 0x24d,
    0x3cb, 0x6a8, 0x12f9, 0x15ba, 0x1861, 0x2a89, 0x1c5c, 0xbe6, 0xc1e, 0x2024, 0x207, 0x19ce,
    0x2710, 0x1744, 0x18bc, 0x2cd7, 0x396, 0x18d5, 0x1c45, 0xc4, 0x21a6, 0xe03, 0x2b3c, 0x2d91,
    0xc5d, 0x432, 0x1fbc, 0xcae, 0x2512, 0x2979, 0x3b2, 0x714, 0xb2e, 0x1a97, 0x1a03, 0x1bcd,
    0x2216, 0x2701, 0xa, 0x263c, 0x1179, 0x200c, 0x2d08, 0x1c34, 0x291, 0x2c99, 0x2a5a, 0x723,
    0xb1d, 0x1ccc, 0x1fb6, 0x2f58, 0x2bfe, 0x1cda, 0x2a0, 0x5f1, 0x2de, 0x1fc7, 0x1ea8, 0x1719,
    0x2fa7, 0x27ec, 0x20ff, 0x12c0, 0x1ac1, 0x2232, 0x2f9b, 0xd3e, 0x2aed, 0x15f0, 0x11e8, 0xed0,
    0x26a, 0x1de5, 0xa3f, 0xf43, 0xebf, 0x204e, 0xac7, 0x2d9c, 0x5ea, 0x25d1, 0xb6, 0x49c, 0x995,
    0x2555, 0x26e2, 0x100, 0x1878, 0x5aa, 0x2e10, 0x271c, 0xcb, 0x1b4c, 0x2fb8, 0x25b7, 0x1543,
    0x2c7b, 0x241a, 0x2223, 0x20ca, 0x24ed, 0x137, 0x1b65, 0x1dc2, 0x7c7, 0x2ec3, 0xd0c, 0x1169,
    0x1c7a, 0x1ea1, 0xf89, 0x2199, 0x291d, 0x1088, 0x2046, 0x256d, 0x2bc7, 0x2e9b, 0x41f, 0x1b55,
    0x2b38, 0xd0, 0x2e6a, 0x1755, 0x6bc, 0x2724, 0x3ba, 0x222e, 0x2c5c, 0x2da5, 0x213c, 0x10fe,
    0x169a, 0x1552, 0x5d3, 0x300, 0x1b5d, 0x1342, 0x2004, 0x256f, 0x2039, 0x667, 0x23b5, 0x1123,
    0xdb, 0x2da0, 0xe1e, 0x2f54, 0x2767, 0x154a, 0x40a, 0x11d3, 0x2821, 0xc09, 0x974, 0x694, 0xfbf,
    0x27ba, 0x132, 0x83f, 0x2d06, 0x10e, 0x183f, 0x29ae, 0x28c3, 0x2dc9, 0x1144, 0x2c70, 0x2a4a,
    0xf3c, 0x1e32, 0x1171, 0x1e43, 0xdd4, 0x2ddf, 0x28d2, 0xfac, 0x3c4, 0x2f19, 0x10a6, 0x2f7,
    0xe1d, 0x828, 0x138f, 0x1332, 0xfab, 0xcf6, 0x13f8, 0x24a0, 0x112d, 0x2717, 0x6e7, 0x1044,
    0x36e, 0xfe8, 0x6a, 0xba7, 0x1d69, 0x29ec, 0x23b2, 0xaee, 0x16df, 0x1068, 0x1a7e, 0x253f,
    0x24c, 0xb33, 0x2683, 0x15ce, 0x1ad3, 0x1a36, 0xc96, 0xaea, 0x260a, 0xce, 0x28b1, 0xe4f,
    0x2b11, 0x5f8, 0x1fc4, 0xe77, 0x2366, 0x11f9, 0x153c, 0x24eb, 0x20cd, 0x1398, 0x22, 0x2b97,
    0x249b, 0x8eb, 0x12b2, 0x2fe3, 0x29c1, 0x1b00, 0x2663, 0xeaa, 0x2e06, 0xe0, 0x1569, 0x10f5,
    0x284e, 0xa38, 0x201d, 0x1c53, 0x1681, 0x1f6f, 0x2f95, 0x2fe8, 0xacb, 0x1680, 0x17fd, 0x2c39,
    0x165a, 0x10bb, 0x29d8, 0x2622, 0x1196, 0x884, 0x2a79, 0x140e, 0x2d80, 0x6fa, 0x11b2, 0x26c4,
    0x355, 0x1054, 0x29e9, 0x23ed, 0xbe3, 0x24fa, 0x1fb3, 0x10ac, 0x2919, 0x2584, 0x10a4, 0xe85,
    0x650, 0x1893, 0x1dc1, 0xd8e, 0x12dc, 0x2d42, 0x284d, 0xfff, 0x250f, 0xacd, 0x13c3, 0x6cc,
    0x1a79, 0x1221, 0x2614, 0x270a, 0x1ea, 0x155, 0x2818, 0x222c, 0x2e5b, 0x25d8, 0x1dbf, 0x191c,
    0xb0f, 0xdac, 0x1082, 0x12ef, 0x11b6, 0xfa8, 0x2b72, 0x159d, 0x209e, 0x31b, 0x2c7c, 0x14f7,
    0xe09, 0x1bb2, 0x1ec7, 0x2404, 0x20ae, 0x6ad, 0xed6, 0x2b70, 0x1c7b, 0x18d1, 0x2732, 0x12da,
    0xd56, 0x5c1, 0x1648, 0x18b7, 0x1605, 0x1bc4, 0x280, 0x2ece, 0xc, 0x1aae, 0x1c4, 0x1cdb,
    0x22d6, 0x21d8, 0x257c, 0x51f, 0x211b, 0xff, 0x2ee0, 0x2585, 0xe1, 0x2c35, 0x26db, 0x2971,
    0x2208, 0x17e1, 0x21be, 0x135e, 0x28d6, 0x2891, 0x1689, 0x2138, 0xb86, 0x2e3a, 0x1204, 0x2d10,
    0x2324, 0xf3f, 0x2508, 0x33d, 0xcb2, 0x292a, 0xe27, 0x2e64, 0x29f8, 0x2d46, 0x9b7, 0x20eb,
    0x1b7c, 0x9eb, 0x2b2a, 0x58c, 0x27d0, 0x121b, 0x272e, 0x29f6, 0x2dbd, 0x2697, 0x2aac, 0xd6f,
    0x1c67, 0x2c5b, 0x108d, 0x363, 0x249d, 0x2d5e, 0x2fd, 0x2cb2, 0x1f8f, 0x20a4, 0xa19, 0x2ac9,
    0x19b1, 0x1581, 0x17a2, 0x29eb, 0x1b72, 0x13b0, 0xee4, 0xa8f, 0x2315, 0x5e6, 0x951, 0x2e29,
    0xdad, 0x1f2b, 0x224e, 0x37f, 0x1a72, 0xa91, 0x1407, 0x2df9, 0x3ad, 0x23f7, 0x1a24, 0x1d2a,
    0x234b, 0x1df3, 0x1143, 0x7ff, 0x1a6d, 0x2774, 0x2690, 0x2ab5, 0x586, 0x2781, 0x2009, 0x2fdd,
    0x2881, 0x399, 0x2fb6, 0x144, 0x137f, 0xfa0, 0x2e4c, 0x1c7f, 0x2fac, 0xb09, 0x1264, 0x127b,
    0x198c, 0x2b40, 0x230, 0x1cf4, 0x180b, 0xb58, 0x144a, 0x2aec, 0xfb, 0x2602, 0x14ee, 0x783,
    0x1098, 0x23d8, 0x203, 0xe9, 0x108a, 0x14b8, 0xeec, 0xc58, 0x1248, 0x243c, 0x28aa, 0x6bf,
    0x27c4, 0x276e, 0x19b8, 0x1d11, 0x2e16, 0x472, 0x1464, 0x24b9, 0x662, 0x1097, 0x2067, 0x20d6,
    0x171c, 0x4, 0x682, 0x17bb, 0x1186, 0x4f2, 0x3ff, 0x2a43, 0x1dc7, 0x1ae5, 0x8cc, 0x2e7c,
    0x2ef8, 0x2ae0, 0x2904, 0xed4, 0x6c5, 0x14ae, 0xb72, 0x11c3, 0x337, 0x2da3, 0x2916, 0x6d8,
    0x1cf9, 0x10ee, 0x1800, 0x1ae4, 0xa0d, 0x101b, 0x1a8d, 0x2e98, 0x24cd, 0x813, 0x1aa4, 0x9b9,
    0x680, 0x2349, 0x24d1, 0x20f8, 0xe31, 0x249f, 0x216b, 0x12d9, 0x1d21, 0x19db, 0x191a, 0x1dd0,
    0x5df, 0x55c, 0x2b86, 0x213, 0xe9e, 0x1ef1, 0x268a, 0x1d5e, 0x1e20, 0x28c1, 0x1379, 0x249,
    0x19de, 0x18b, 0x1e41, 0x2a1e, 0x2612, 0x297, 0x2e96, 0x2102, 0x46, 0x1b9f, 0x1a4d, 0x2050,
    0x1b32, 0x568, 0x11f7, 0x1829, 0x870, 0x1f4, 0x1dca, 0x990, 0x1df6, 0x2b62, 0x13ec, 0x9f2,
    0x1260, 0x2997, 0x1412, 0x1e6d, 0x1694, 0x11ac, 0x2d8b, 0x276f, 0x26f5, 0x233e, 0x2b44, 0x2f5a,
    0x2d37, 0x2cb1, 0xc75, 0x98d, 0x1d56, 0x7ae, 0x10e6, 0x113f, 0x17b8, 0xad3, 0x737, 0x221e,
    0x1b70, 0x1f3e, 0x2966, 0x18b2, 0x4fa, 0x2044, 0x1312, 0x154e, 0x2029, 0x700, 0x1b45, 0x27a6,
    0x226a, 0x21bf, 0x58d, 0x2f11, 0x2e02, 0x17fc, 0x4d2, 0x1757, 0xcb1, 0x2ef1, 0x2582, 0x1276,
    0x881, 0x2fc0, 0x104a, 0x670, 0x274f, 0x2b53, 0x19dd, 0x752, 0x1663, 0xcbd, 0x2b2b, 0x2fc6,
    0x13b6, 0x21e6, 0x15f6, 0x126b, 0x2637, 0x1cd9, 0x2f50, 0xe82, 0x5b0, 0x24e0, 0x1350, 0x2f24,
    0x21f7, 0x1a16, 0x2f3e, 0x167e, 0x1f7d, 0x28a0, 0x16f0, 0xe33, 0x53b, 0x28c5, 0x1500, 0x2f88,
    0x26cc, 0x2018, 0x1604, 0x218b, 0x2cd1, 0x9ee, 0x17f3, 0x5fd, 0x1f5a, 0x2d0, 0x2b46, 0x23cc,
    0x503, 0x1c46, 0x1cc3, 0x28e2, 0x243e, 0x122b, 0x2e0c, 0xe37, 0x2611, 0x85e, 0x9b8, 0x1b24,
    0x762, 0x19b6, 0x3bc, 0x2d50, 0x2079, 0x18da, 0x170a, 0x800, 0xaa2, 0x135a, 0x1a15, 0x13d1,
    0xca, 0x2113, 0x2db9, 0xdb2, 0x1a5c, 0x29a9, 0x1488, 0x14c1, 0x2c9, 0x917, 0x28e7, 0x265c,
    0xdab, 0x2ab9, 0x2bc6, 0x105b, 0x1839, 0x219c, 0x50, 0x11da, 0x1802, 0xf56, 0x2e6, 0x2190,
    0xddb, 0x56e, 0x9d9, 0x1c81, 0x1016, 0x12d6, 0x296f, 0x14b4, 0x1014, 0x1e64, 0x1d90, 0x89f,
    0x2bc2, 0x2777, 0x2819, 0x1c65, 0x1a41, 0x5a2, 0x2cd2, 0x427, 0xd71, 0x29c8, 0x1e58, 0x53f,
    0x7c5, 0x1dcd, 0x4a1, 0x1268, 0x2597, 0x2926, 0xee, 0x111b, 0x1038, 0xe6c, 0x22dc, 0x2f2f,
    0x441, 0x2cfd, 0x1cb0, 0x6a4, 0x2224, 0x620, 0x5dc, 0x16b1, 0x2a1d, 0x1787, 0x20c7, 0x641,
    0xd84, 0x1c05, 0x2d0d, 0x2f52, 0x1b8c, 0xd7d, 0x17e8, 0x1589, 0xc73, 0x151b, 0x4e2, 0x1ae9,
    0x1b18, 0xb9b, 0x949, 0x2c60, 0x1e7a, 0xd5, 0x1bdc, 0x1f57, 0x1753, 0x124a, 0x559, 0xb76,
    0x2334, 0x12d1, 0x1de1, 0x14b2, 0x2faa, 0x1697, 0x147a, 0x5a1, 0x2c30, 0x1c02, 0x1043, 0x2ee1,
    0x2402, 0x1cc8, 0x2a16, 0xff7, 0x1364, 0x1b9a, 0x2a53, 0x2f94, 0x294c, 0x1ee5, 0x1a87, 0x2141,
    0xd66, 0x953, 0x28a3, 0x2f30, 0x2477, 0x18e3, 0x1035, 0x1fc1, 0x1d68, 0x2fb3, 0x138c, 0x2487,
    0x1bf8, 0xd96, 0x1018, 0x748, 0x244e, 0x15bd, 0x175e, 0x2be, 0x23d, 0x1da, 0x176d, 0xc17,
    0x24be, 0x2ebb, 0x7d8, 0x100a, 0x759, 0x1db4, 0x2259, 0x23f4, 0x2d59, 0x2847, 0xbf5, 0x1cfe,
    0xa20, 0x258, 0x1180, 0x279c, 0x54, 0x2abf, 0xc5c, 0x9f9, 0x3d5, 0x2ce4, 0x165f, 0x23d9,
    0x27b9, 0x6f9, 0x281a, 0x169e, 0x627, 0x156d, 0x1ff8, 0x211, 0x2e34, 0x1724, 0x2c2e, 0x2790,
    0x2dd5, 0x2bf2, 0xdbc, 0x2884, 0x20a9, 0x2390, 0x1e1a, 0x1b6a, 0x5f7, 0xab7, 0x1333, 0x16ab,
    0x28dd, 0x20, 0x30f, 0x24b6, 0x5c2, 0x1ce4, 0x1400, 0x2669, 0x60, 0x156c, 0xe20, 0x26d4,
    0x26ab, 0x1ebb, 0x223d, 0x5b4, 0x2025, 0x1e1c, 0xaae, 0x2e08, 0x6cd, 0x1677, 0x13d9, 0x17b5,
    0x1046, 0x1d8c, 0x14eb, 0x18d8, 0x1ce5, 0x2478, 0x16ae, 0xb79, 0x23d4, 0x684, 0x156b, 0x567,
    0x1a, 0x29ce, 0x83a, 0x19e8, 0x58e, 0x294a, 0x1136, 0x2319, 0x2fba, 0x1a29, 0x1d, 0x1879,
    0x291b, 0x19f6, 0x2c2f, 0x21c9, 0x19bb, 0xbbc, 0x26f9, 0xc22, 0x708, 0x11a1, 0x18d3, 0x7f8,
    0x28f8, 0x2427, 0x1deb, 0xaed, 0x26aa, 0x2482, 0x203b, 0x2f05, 0x2b82, 0x192f, 0x2df4, 0x8dc,
    0x2877, 0xd5e, 0x240e, 0x775, 0x2dae, 0x1d3e, 0x20ba, 0x215b, 0x22d1, 0xeba, 0xf50, 0xaa8,
    0x184a, 0x1f67, 0x2e04, 0xc6e, 0x6dd, 0x1a09, 0x27f, 0x494, 0x1426, 0xae3, 0xe15, 0x65f,
    0x13c4, 0x105, 0x872, 0x2667, 0x1ff6, 0xd9f, 0x2ca1, 0x2f39, 0x2657, 0x23fd, 0x2405, 0xb73,
    0x2294, 0x1f1e, 0x2eba, 0x110a, 0x2cae, 0x141f, 0x22cd, 0x25d6, 0x11c1, 0x1c, 0x2d8e, 0x161a,
    0x1aa8, 0x229e, 0x1bf9, 0x7cf, 0x106d, 0x2c40, 0xd93, 0x255e, 0x28c2, 0xc1a, 0x2f17, 0x7ca,
    0x2f63, 0xbf,
];
const IROOTS: [i32; 1024] = [
    0x2ac8, 0x452, 0x297c, 0x666, 0xb4c, 0x2b8, 0x1a74, 0xfd, 0x1a47, 0x1d08, 0x2959, 0x2c36,
    0x2db4, 0x56c, 0x254e, 0x1125, 0x2f3d, 0x13bc, 0x172c, 0x2c6b, 0x32a, 0x1745, 0x18bd, 0x8f1,
    0x1633, 0x2dfa, 0xfdd, 0x23e3, 0x241b, 0x13a5, 0x578, 0x17a0, 0xa9, 0x104b, 0x1335, 0x24e4,
    0x28de, 0x5a7, 0x368, 0x2d70, 0x13cd, 0x2f9, 0xff5, 0x1e88, 0x9c5, 0x2ff7, 0x900, 0xdeb,
    0x1434, 0x15fe, 0x156a, 0x24d3, 0x28ed, 0x2c4f, 0x688, 0xaef, 0x2353, 0x1045, 0x2bcf, 0x23a4,
    0x270, 0x4c5, 0x21fe, 0xe5b, 0xfbb, 0x1f79, 0x6e4, 0xe68, 0x2078, 0x1160, 0x1387, 0x1e98,
    0x22f5, 0x13e, 0x283a, 0x123f, 0x149c, 0x2eca, 0xb14, 0xf37, 0xdde, 0xbe7, 0x386, 0x1abe,
    0xa4a, 0x49, 0x14b5, 0x2f36, 0x8e5, 0x1f1, 0x2a57, 0x1789, 0x2f01, 0x91f, 0xaac, 0x266c,
    0x2b65, 0x2f4b, 0xa30, 0x2a17, 0x265, 0x253a, 0xfb3, 0x2142, 0x20be, 0x25c2, 0x121c, 0x2d97,
    0x2131, 0x1e19, 0x1a11, 0x514, 0x22c3, 0x66, 0xdcf, 0x1540, 0x1d41, 0xf02, 0x815, 0x5a, 0x18e8,
    0x1159, 0x103a, 0x2d23, 0x2a10, 0x2d61, 0x1327, 0x403, 0x25c9, 0x7b3, 0x1f0c, 0x1a98, 0x2f21,
    0x1fb, 0x2157, 0x99e, 0x1501, 0x640, 0x1e, 0x1d4f, 0x2716, 0xb66, 0x46a, 0x2fdf, 0x1c69, 0xf34,
    0xb16, 0x1ac5, 0x1e08, 0xc9b, 0x218a, 0x103d, 0x2a09, 0x4f0, 0x21b2, 0x750, 0x2f33, 0x9f7,
    0x2517, 0x236b, 0x15cb, 0x152e, 0x1a33, 0x97e, 0x24ce, 0x2db5, 0xac2, 0x1583, 0x1f99, 0x1922,
    0x2513, 0xc4f, 0x615, 0x1298, 0x245a, 0x2f97, 0x2019, 0x2c93, 0x1fbd, 0x291a, 0x8ea, 0x1ed4,
    0xb61, 0x1c09, 0x230b, 0x2056, 0x1ccf, 0x1c72, 0x27d9, 0x21e4, 0x2d0a, 0x1f5b, 0xe8, 0x2c3d,
    0x2055, 0x72f, 0x222, 0x222d, 0x11be, 0x1e90, 0x11cf, 0x20c5, 0x5b7, 0x391, 0x1ebd, 0x238,
    0x73e, 0x653, 0x17c2, 0x2ef3, 0x2fb, 0x27c2, 0x2ecf, 0x847, 0x2042, 0x296d, 0x268d, 0x23f8,
    0x7e0, 0x1e2e, 0x2bf7, 0x1ab7, 0x89a, 0xad, 0x21e3, 0x261, 0x2f26, 0x1ede, 0xc4c, 0x299a,
    0xfc8, 0xa92, 0xffd, 0x1cbf, 0x14a4, 0x2d01, 0x2a2e, 0x1aaf, 0x1967, 0x1f03, 0xec5, 0x25c,
    0x3a5, 0xdd3, 0x2c47, 0x8dd, 0x2945, 0x18ac, 0x197, 0x2f31, 0x4c9, 0x14ac, 0x2be2, 0x166,
    0x43a, 0xa94, 0x1b53, 0x293c, 0x212d, 0x6fd, 0x521, 0x109, 0x185, 0x2735, 0x151c, 0x123a,
    0x5be, 0x2c02, 0x2b0f, 0x1e7b, 0x1846, 0x297f, 0x2ffd, 0x18e5, 0xf2b, 0xf9a, 0x1f6a, 0x299f,
    0xb48, 0x1b9d, 0x2b8f, 0x1eb, 0x12f0, 0x1649, 0x893, 0x83d, 0x2942, 0x757, 0xbc5, 0x1db9,
    0x23a9, 0x2115, 0x1b49, 0x1f77, 0x2f18, 0x2dfe, 0xc29, 0x1f69, 0x287e, 0x1b13, 0x9ff, 0x2f06,
    0x515, 0x1bb7, 0x24a9, 0x17f6, 0x130d, 0x2dd1, 0x4c1, 0x1675, 0x1d86, 0x1d9d, 0x24f8, 0x55,
    0x1382, 0x1b5, 0x2061, 0x1c82, 0x2ebd, 0x4b, 0x2c68, 0x780, 0x24, 0xff8, 0x880, 0x2a7b, 0x54c,
    0x971, 0x88d, 0x1594, 0x2802, 0x1ebe, 0x120e, 0xcb6, 0x12d7, 0x15dd, 0xc0a, 0x2c54, 0x208,
    0x1bfa, 0x2570, 0x158f, 0x2c82, 0xdb3, 0x10d6, 0x2254, 0x1d8, 0x26b0, 0x2a1b, 0xcec, 0x2572,
    0x211d, 0x1c51, 0x148f, 0x616, 0x185f, 0x1a80, 0x1650, 0x538, 0x25e8, 0xf5d, 0x1072, 0x34f,
    0x2d04, 0x2a3, 0xb64, 0x2c9e, 0x1f74, 0x3a6, 0x139a, 0x2292, 0x555, 0x96a, 0x244, 0x60b, 0x8d3,
    0x1de6, 0x831, 0x2a75, 0x4d7, 0x2616, 0x1485, 0xf16, 0x264a, 0x2bb, 0x609, 0x19d, 0x21da,
    0x6d7, 0x234f, 0x2cc4, 0xaf9, 0x20c2, 0xcdd, 0x2f1, 0x1dfd, 0x1c7, 0x247b, 0xec9, 0x1978,
    0x770, 0x72b, 0x1ca3, 0xe43, 0x1820, 0xdf9, 0x690, 0x926, 0x3cc, 0x2f20, 0xa7c, 0x121, 0x2f02,
    0xee6, 0x2ae2, 0xa85, 0xe29, 0xd2b, 0x1326, 0x2e3d, 0x1553, 0x2ff5, 0x133, 0x2d81, 0x143d,
    0x19fc, 0x174a, 0x19b9, 0x2a40, 0x22ab, 0x1d27, 0x8cf, 0x1730, 0x1386, 0x491, 0x212b, 0x2954,
    0xf53, 0xbfd, 0x113a, 0x144f, 0x21f8, 0x1b0a, 0x385, 0x2ce6, 0xf63, 0x1a64, 0x48f, 0x2059,
    0x1e4b, 0x1d12, 0x1f7f, 0x2255, 0x24f2, 0x16e5, 0x1242, 0xa29, 0x1a6, 0xdd5, 0x7e9, 0x2eac,
    0x2e17, 0x8f7, 0x9ed, 0x1de0, 0x1588, 0x2935, 0x1c3e, 0x2534, 0xaf2, 0x2002, 0x7b4, 0x2bf,
    0x1d25, 0x2273, 0x1240, 0x176e, 0x29b1, 0x217c, 0x1f5d, 0xa7d, 0x6e8, 0x1f55, 0x104e, 0xb07,
    0x241e, 0xc14, 0x618, 0x1fad, 0x2cac, 0x93d, 0x1e4f, 0x2907, 0x281, 0x1bf3, 0x588, 0x277d,
    0x1e6b, 0x9df, 0x629, 0x1f46, 0x19a7, 0x3c8, 0x1804, 0x1981, 0x2536, 0x19, 0x6c, 0x1092,
    0x1980, 0x13ae, 0xfe4, 0x2f42, 0x9e, 0x2837, 0xea, 0x23e7, 0x73f, 0xaa3, 0x226e, 0x3c1, 0x1f94,
    0x2832, 0x1408, 0xd63, 0x1559, 0x19e7, 0x273, 0x2fe5, 0x1e40, 0xa2b, 0xd34, 0x1be2, 0x353,
    0x1ef7, 0x147, 0x10e3, 0xd6d, 0x248e, 0xbfc, 0xc04, 0x9aa, 0xc8, 0x360, 0x2262, 0x100b, 0x99a,
    0x278f, 0x2efc, 0x1c3d, 0x29a2, 0x21ec, 0x251e, 0x1bdb, 0x2b6d, 0x2d82, 0x15f8, 0x2924, 0x2393,
    0x1fd, 0x109a, 0x17b7, 0x2559, 0x20b1, 0x2147, 0xd30, 0xea6, 0xf47, 0x12c3, 0x253, 0x288c,
    0xbf3, 0x22a3, 0x78a, 0x2725, 0x20d, 0x16d2, 0x47f, 0xfc, 0xfc6, 0xb7f, 0x957, 0x2514, 0x1216,
    0xbda, 0x709, 0x2809, 0x172e, 0x1e60, 0x28f9, 0x23df, 0x908, 0x2445, 0x1646, 0xe38, 0x3d2,
    0x160b, 0x6e6, 0x1788, 0x2fe4, 0x15d8, 0x47, 0xce8, 0x1ecb, 0x6b7, 0x2a73, 0x1619, 0x27c7,
    0x633, 0x2fe7, 0x2a9a, 0x1a96, 0x297d, 0xc2d, 0x2488, 0x1953, 0xb89, 0x131c, 0x1729, 0x1b16,
    0x1275, 0x1fbb, 0x184c, 0x1c28, 0x198a, 0x2934, 0x1f9, 0x2553, 0x11e5, 0xfdc, 0x2a4d, 0xdc4,
    0x1146, 0x956, 0x92d, 0x21e1, 0x1a95, 0x2fa1, 0x998, 0x1c01, 0x131d, 0x2a3f, 0xb4b, 0x2cf2,
    0x2fe1, 0x724, 0x1956, 0x1cce, 0x254a, 0x2a0a, 0x1497, 0x11e7, 0xc71, 0xf58, 0x77d, 0x2245,
    0x40f, 0x22c, 0x871, 0x3d3, 0x18dd, 0x1cd, 0x2df0, 0x1009, 0x1a94, 0x29da, 0x1963, 0x7e7,
    0x2908, 0x848, 0xc28, 0x19a2, 0x31d, 0x2c2c, 0x2608, 0x23a5, 0x542, 0x2fad, 0x865, 0x1e81,
    0x2da9, 0x25e1, 0x1303, 0x240c, 0x7ba, 0x2a8, 0xc0d, 0xda8, 0x124d, 0x28a8, 0x1ff7, 0x2829,
    0x146, 0xb43, 0x23ea, 0x1894, 0x2e27, 0x2dc4, 0x2d43, 0x18a3, 0x1a44, 0xbb3, 0x28b9, 0x1fe9,
    0x226b, 0x1409, 0xb7a, 0x1c75, 0x4e, 0x1299, 0x1040, 0x1fcc, 0x171e, 0xb8a, 0xd1, 0x75e,
    0x26ae, 0x229b, 0xec0, 0x157a, 0x111c, 0x6b5, 0x6d, 0x5ae, 0x1467, 0x1c9d, 0x200a, 0x5eb,
    0x1339, 0xbff, 0x120, 0x1fbe, 0x13ff, 0x3d1, 0x2a60, 0x1b87, 0x196a, 0x57, 0x1b4f, 0x1220,
    0x1d30, 0xccd, 0x248b, 0x2aa8, 0x1db7, 0x18ae, 0x10aa, 0x1425, 0x2f2c, 0x1187, 0x3a1, 0x26b8,
    0x2466, 0x14e9, 0x1518, 0x2b1f, 0x1ae6, 0x238e, 0x1a78, 0x1819, 0x2284, 0x1475, 0xaf, 0x2f4,
    0x13fc, 0x227d, 0x29c0, 0xf3a, 0x187a, 0x5e4, 0x1950, 0x2a25, 0x29e1, 0xddd, 0x295d, 0x1351,
    0x304, 0x2bc0, 0xd2, 0xd25, 0x2195, 0x1fc9, 0x1ee6, 0x2f13, 0x6db, 0xa6a, 0x1d99, 0x2b60,
    0x1234, 0x283c, 0x2ac2, 0x11a9, 0x639, 0x2290, 0x2bda, 0x32f, 0x2a5f, 0x15c0, 0x139c, 0x7e8,
    0x88a, 0x43f, 0x2762, 0x1271, 0x119d, 0x1fed, 0x1b4d, 0x692, 0x1d2b, 0x1feb, 0x1380, 0x2628,
    0x2a93, 0x2226, 0xe71, 0x2d1b, 0x20ab, 0x17ff, 0x1e27, 0x2fb1, 0xe65, 0x17c8, 0x1fa6, 0x43b,
    0x548, 0x2256, 0x9a5, 0x71a, 0x26ea, 0x2d38, 0x1b40, 0x1b79, 0x658, 0x15a5, 0x224f, 0x248,
    0xeee, 0x2f37, 0x1c30, 0x15ec, 0x1ca7, 0x255f, 0x2801, 0x18f7, 0x1727, 0xf88, 0x2b1, 0x2c45,
    0x164b, 0x289f, 0x14dd, 0x2649, 0x27a3, 0x9f0, 0x21ca, 0x1f5, 0x1dd6, 0xbc3, 0x71f, 0x133e,
    0x13bb, 0x2afe, 0xc35, 0x4bb, 0x2d31, 0x10a7, 0x2a04, 0x180e, 0x2613, 0x330, 0xe76, 0x19fd,
    0xfe9, 0x935, 0x79, 0x1b01, 0x73c, 0x2ac6, 0x21ce, 0x1911, 0x761, 0x1084, 0x1983, 0xc3, 0x15eb,
    0xe0a, 0xdd, 0x1cb1, 0xb21, 0x2a51, 0x217f, 0xb1, 0x1328, 0x9ca, 0x1d96, 0x1a0b, 0xe1b, 0x1c4b,
    0x3b, 0x4d6, 0x2344, 0x199e, 0x28af, 0x1624, 0x4ae, 0x8b2, 0x2991, 0x1fb7, 0x41, 0x2780,
    0x1d8b, 0xa7f, 0x110, 0x2350, 0x18aa, 0x2b2f, 0x1805, 0x1ff, 0xf0, 0x2a74, 0xe42, 0xd97, 0x85b,
    0x14bc, 0x2901, 0xfd8, 0x1ab3, 0x1cef, 0xfbd, 0x2b07, 0x174f, 0x69b, 0x10c3, 0x1491, 0xde3,
    0x28ca, 0x252e, 0x1849, 0x1ec2, 0x1f1b, 0x2853, 0x12ab, 0x2674, 0x238c, 0x350, 0x2ca, 0xa7,
    0x4bd, 0xcc3, 0x90c, 0x892, 0x276, 0x1e55, 0x196d, 0x1194, 0x1bef, 0x66a, 0x1da1, 0x260f,
    0x1c15, 0x49f, 0x120b, 0x2671, 0x1237, 0x2e0d, 0x2791, 0x17d8, 0x1e0a, 0x2a99, 0x14cf, 0xfb1,
    0x15b4, 0x1462, 0x2fbb, 0xeff, 0x16b, 0x2d6a, 0x9ef, 0x5e3, 0x11c0, 0x2e76, 0x1623, 0x2db8,
    0x1c88, 0x740, 0x11e1, 0x12a3, 0x977, 0x1110, 0x2163, 0x2dee, 0x47b, 0x2aa5, 0x2a22, 0x1231,
    0x16e7, 0x1626, 0x12e0, 0x1d28, 0xe96, 0xb62, 0x21d0, 0xf09, 0xb30, 0xcb8, 0x2981, 0x2648,
    0x155d, 0x27ee, 0xb34, 0x169, 0x1574, 0x1fe6, 0x25f4, 0x151d, 0x1801, 0x1f13, 0x1308, 0x2929,
    0x6eb, 0x25e, 0x2cca, 0x1e3e, 0x248f,
];

fn round(a: i32, b: i32) -> i32 {
    return (a + b / 2) / b;
}

/* Constant time absolute value */
fn nabs(x: i32) -> i32 {
    let mask = x >> 31;
    return (x + mask) ^ mask;
}

/* Montgomery stuff */

fn redc(t: u64) -> i32 {
    let m = (t as u32).wrapping_mul(ND);
    return (((m as u64) * (PRIME as u64) + t) >> WL) as i32;
}

fn nres(x: i32) -> i32 {
    return redc((x as u64) * R2MODP);
}

fn modmul(a: i32, b: i32) -> i32 {
    return redc((a as u64) * (b as u64));
}

/* Cooley-Tukey NTT */
fn ntt(x: &mut [i32]) {
    let mut t = DEGREE / 2;
    let q = PRIME;

    /* Convert to Montgomery form */
    for j in 0..DEGREE {
        x[j] = nres(x[j])
    }
    let mut m = 1;
    while m < DEGREE {
        let mut k = 0;
        for i in 0..m {
            let s = ROOTS[m + i];
            for j in k..k + t {
                let u = x[j];
                let v = modmul(x[j + t], s);
                x[j] = u + v;
                x[j + t] = u + 2 * q - v;
            }
            k += 2 * t;
        }
        t /= 2;
        m *= 2;
    }
}

/* Gentleman-Sande INTT */

fn intt(x: &mut [i32]) {
    let mut t = 1;
    let q = PRIME;
    let mut m = DEGREE / 2;
    while m > 1 {
        let mut k = 0;
        for i in 0..m {
            let s = IROOTS[m + i];
            for j in k..k + t {
                let u = x[j];
                let v = x[j + t];
                x[j] = u + v;
                let w = u + (DEGREE as i32) * q - v;
                x[j + t] = modmul(w, s);
            }
            k += 2 * t;
        }
        t *= 2;
        m /= 2;
    }

    /* Last iteration merged with n^-1 */
    t = DEGREE / 2;
    for j in 0..t {
        let u = x[j];
        let v = x[j + t];
        let w = u + (DEGREE as i32) * q - v;
        x[j + t] = modmul(w, INVPR);
        x[j] = modmul(u + v, INV);
    }
    /* convert back from Montgomery to "normal" form */
    for j in 0..DEGREE {
        x[j] = redc(x[j] as u64);
        x[j] -= q;
        x[j] += (x[j] >> (WL - 1)) & q;
    }
}

/* See https://eprint.iacr.org/2016/1157.pdf */

fn encode(key: &[u8], poly: &mut [i32]) {
    let q2 = PRIME / 2;
    let mut j = 0;
    let mut i = 0;
    while i < 256 {
        let mut kj = key[j];
        j += 1;
        for _ in 0..8 {
            let b = i32::from(kj & 1);
            poly[i] = b * q2;
            poly[i + 256] = b * q2;
            poly[i + 512] = b * q2;
            poly[i + 768] = b * q2;
            kj >>= 1;
            i += 1;
        }
    }
}

fn decode(poly: &[i32], key: &mut [u8]) {
    let q2 = PRIME / 2;
    for i in 0..32 {
        key[i] = 0;
    }

    let mut i = 0;
    let mut j = 0;
    while i < 256 {
        for _ in 0..8 {
            let t = nabs(poly[i] - q2)
                + nabs(poly[i + 256] - q2)
                + nabs(poly[i + 512] - q2)
                + nabs(poly[i + 768] - q2);
            let mut b = t - PRIME;
            b = (b >> 31) & 1;
            key[j] = (key[j] >> 1) + ((b << 7) as u8);
            i += 1;
        }
        j += 1;
    }
}

/* convert 32-byte seed to random polynomial */

fn parse(seed: &[u8], poly: &mut [i32]) {
    let mut hash: [u8; 4 * DEGREE] = [0; 4 * DEGREE];
    let mut sh = SHA3::new(sha3::SHAKE128);
    for i in 0..32 {
        sh.process(seed[i])
    }
    sh.shake(&mut hash, 4 * DEGREE);

    let mut j = 0;
    for i in 0..DEGREE {
        let mut n = i32::from(hash[j] & 0x7f);
        n <<= 8;
        n += i32::from(hash[j + 1]);
        n <<= 8;
        n += i32::from(hash[j + 2]);
        n <<= 8;
        n += i32::from(hash[j + 3]);
        j += 4;
        poly[i] = nres(n);
        //poly[i]=modmul(n,ONE); // reduce 31-bit random number mod q
    }
}

/* Compress 14 bits polynomial coefficients into byte array */
/* 7 bytes is 3x14 */

fn nhs_pack(poly: &[i32], array: &mut [u8]) {
    let mut j = 0;
    let mut i = 0;
    while i < DEGREE {
        let a = poly[i];
        let b = poly[i + 1];
        let c = poly[i + 2];
        let d = poly[i + 3];
        i += 4;
        array[j] = (a & 0xff) as u8;
        array[j + 1] = (((a >> 8) | (b << 6)) & 0xff) as u8;
        array[j + 2] = ((b >> 2) & 0xff) as u8;
        array[j + 3] = (((b >> 10) | (c << 4)) & 0xff) as u8;
        array[j + 4] = ((c >> 4) & 0xff) as u8;
        array[j + 5] = (((c >> 12) | (d << 2)) & 0xff) as u8;
        array[j + 6] = (d >> 6) as u8;
        j += 7;
    }
}

fn nhs_unpack(array: &[u8], poly: &mut [i32]) {
    let mut j = 0;
    let mut i = 0;
    while i < DEGREE {
        let a = ((array[j]) & 0xff) as i32;
        let b = ((array[j + 1]) & 0xff) as i32;
        let c = ((array[j + 2]) & 0xff) as i32;
        let d = ((array[j + 3]) & 0xff) as i32;
        let e = ((array[j + 4]) & 0xff) as i32;
        let f = ((array[j + 5]) & 0xff) as i32;
        let g = ((array[j + 6]) & 0xff) as i32;
        j += 7;
        poly[i] = a | ((b & 0x3f) << 8);
        poly[i + 1] = (b >> 6) | (c << 2) | ((d & 0xf) << 10);
        poly[i + 2] = (d >> 4) | (e << 4) | ((f & 3) << 12);
        poly[i + 3] = (f >> 2) | (g << 6);
        i += 4;
    }
}

/* See https://eprint.iacr.org/2016/1157.pdf */

fn compress(poly: &[i32], array: &mut [u8]) {
    let mut col = 0 as i32;
    let mut j = 0;
    let mut i = 0;
    while i < DEGREE {
        for _ in 0..8 {
            let b = round(poly[i] * 8, PRIME) & 7;
            col = (col << 3) + b;
            i += 1;
        }
        array[j] = (col & 0xff) as u8;
        array[j + 1] = ((col >> 8) & 0xff) as u8;
        array[j + 2] = ((col >> 16) & 0xff) as u8;
        j += 3;
        col = 0;
    }
}

fn decompress(array: &[u8], poly: &mut [i32]) {
    let mut j = 0;
    let mut i = 0;
    while i < DEGREE {
        let mut col = (array[j + 2] as i32) & 0xff;
        col = (col << 8) + ((array[j + 1] as i32) & 0xff);
        col = (col << 8) + ((array[j] as i32) & 0xff);
        j += 3;
        for _ in 0..8 {
            let b = (col & 0xe00000) >> 21;
            col <<= 3;
            poly[i] = round(b * PRIME, 8);
            i += 1;
        }
    }
}

/* generate centered binomial distribution */

fn error(rng: &mut RAND, poly: &mut [i32]) {
    for i in 0..DEGREE {
        let mut n1 = ((rng.getbyte() as i32) & 0xff) + (((rng.getbyte() as i32) & 0xff) << 8);
        let mut n2 = ((rng.getbyte() as i32) & 0xff) + (((rng.getbyte() as i32) & 0xff) << 8);
        let mut r = 0 as i32;
        for _ in 0..16 {
            r += (n1 & 1) - (n2 & 1);
            n1 >>= 1;
            n2 >>= 1;
        }
        poly[i] = r + PRIME;
    }
}

fn redc_it(p: &mut [i32]) {
    for i in 0..DEGREE {
        p[i] = redc(p[i] as u64);
    }
}

fn nres_it(p: &mut [i32]) {
    for i in 0..DEGREE {
        p[i] = nres(p[i]);
    }
}

fn poly_mul(p1: &mut [i32], p3: &[i32]) {
    for i in 0..DEGREE {
        p1[i] = modmul(p1[i], p3[i]);
    }
}

fn poly_add(p1: &mut [i32], p3: &[i32]) {
    for i in 0..DEGREE {
        p1[i] = p1[i] + p3[i];
    }
}

fn poly_rsub(p1: &mut [i32], p2: &[i32]) {
    for i in 0..DEGREE {
        p1[i] = p2[i] + PRIME - p1[i];
    }
}

/* reduces inputs < 2q */
fn poly_soft_reduce(poly: &mut [i32]) {
    for i in 0..DEGREE {
        let e = poly[i] - PRIME;
        poly[i] = e + ((e >> (WL - 1)) & PRIME);
    }
}

/* fully reduces modulo q */
fn poly_hard_reduce(poly: &mut [i32]) {
    for i in 0..DEGREE {
        let mut e = modmul(poly[i], ONE);
        e = e - PRIME;
        poly[i] = e + ((e >> (WL - 1)) & PRIME);
    }
}

/* API files */

pub fn server_1(mut rng: &mut RAND, sb: &mut [u8], ss: &mut [u8]) {
    let mut seed: [u8; 32] = [0; 32];
    let mut array: [u8; 1792] = [0; 1792];
    let mut s: [i32; DEGREE] = [0; DEGREE];
    let mut e: [i32; DEGREE] = [0; DEGREE];
    let mut b: [i32; DEGREE] = [0; DEGREE];

    for i in 0..32 {
        seed[i] = rng.getbyte();
    }

    parse(&seed, &mut b);

    error(&mut rng, &mut e);
    error(&mut rng, &mut s);

    ntt(&mut s);
    ntt(&mut e);
    poly_mul(&mut b, &s);
    poly_add(&mut b, &e);
    poly_hard_reduce(&mut b);

    redc_it(&mut b);
    nhs_pack(&b, &mut array);

    for i in 0..32 {
        sb[i] = seed[i];
    }

    for i in 0..1792 {
        sb[i + 32] = array[i];
    }

    poly_hard_reduce(&mut s);
    nhs_pack(&s, &mut array);

    for i in 0..1792 {
        ss[i] = array[i];
    }
}

pub fn client(mut rng: &mut RAND, sb: &[u8], uc: &mut [u8], okey: &mut [u8]) {
    let mut sh = SHA3::new(sha3::HASH256);

    let mut seed: [u8; 32] = [0; 32];
    let mut array: [u8; 1792] = [0; 1792];
    let mut key: [u8; 32] = [0; 32];
    let mut cc: [u8; 384] = [0; 384];

    let mut sd: [i32; DEGREE] = [0; DEGREE];
    let mut ed: [i32; DEGREE] = [0; DEGREE];
    let mut u: [i32; DEGREE] = [0; DEGREE];
    let mut k: [i32; DEGREE] = [0; DEGREE];
    let mut c: [i32; DEGREE] = [0; DEGREE];

    error(&mut rng, &mut sd);
    error(&mut rng, &mut ed);

    ntt(&mut sd);
    ntt(&mut ed);

    for i in 0..32 {
        seed[i] = sb[i];
    }

    for i in 0..1792 {
        array[i] = sb[i + 32];
    }

    parse(&seed, &mut u);

    poly_mul(&mut u, &sd);
    poly_add(&mut u, &ed);
    poly_hard_reduce(&mut u);

    for i in 0..32 {
        key[i] = rng.getbyte();
    }

    for i in 0..32 {
        sh.process(key[i]);
    }
    sh.hash(&mut key);

    encode(&key, &mut k);

    nhs_unpack(&array, &mut c);
    nres_it(&mut c);

    poly_mul(&mut c, &sd);
    intt(&mut c);
    error(&mut rng, &mut ed);
    poly_add(&mut c, &ed);
    poly_add(&mut c, &k);

    compress(&c, &mut cc);

    sh = SHA3::new(sha3::HASH256);
    for i in 0..32 {
        sh.process(key[i]);
    }
    sh.hash(&mut key);

    for i in 0..32 {
        okey[i] = key[i];
    }

    redc_it(&mut u);
    nhs_pack(&u, &mut array);

    for i in 0..1792 {
        uc[i] = array[i];
    }

    for i in 0..384 {
        uc[i + 1792] = cc[i];
    }
}

pub fn server_2(ss: &[u8], uc: &[u8], okey: &mut [u8]) {
    let mut sh = SHA3::new(sha3::HASH256);

    let mut s: [i32; DEGREE] = [0; DEGREE];
    let mut k: [i32; DEGREE] = [0; DEGREE];
    let mut c: [i32; DEGREE] = [0; DEGREE];

    let mut array: [u8; 1792] = [0; 1792];
    let mut key: [u8; 32] = [0; 32];
    let mut cc: [u8; 384] = [0; 384];

    for i in 0..1792 {
        array[i] = uc[i];
    }

    nhs_unpack(&array, &mut k);
    nres_it(&mut k);

    for i in 0..384 {
        cc[i] = uc[i + 1792];
    }

    decompress(&cc, &mut c);

    for i in 0..1792 {
        array[i] = ss[i];
    }

    nhs_unpack(&array, &mut s);

    poly_mul(&mut k, &s);
    intt(&mut k);
    poly_rsub(&mut k, &c);
    poly_soft_reduce(&mut k);

    decode(&k, &mut key);

    for i in 0..32 {
        sh.process(key[i]);
    }
    sh.hash(&mut key);

    for i in 0..32 {
        okey[i] = key[i];
    }
}

/*
fn main() {
    let x=3;
    let y=redc(x as u64);
    let z=redc((y as u64)*(R2MODP));
    println!("{:02x}",z);

    let mut a:[i32;1024]=[0;1024];
    for i in 0..1024 {a[i]=i as i32}

    ntt(&mut a);

    for i in 0..1024 {a[i]=modmul(a[i],ONE)}

    intt(&mut a);

    println!("{:02x}",a[7]);

}
*/
