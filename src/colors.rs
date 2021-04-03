#![rustfmt_skip]
pub const COLORS: [f32; 255 * 3] = [
  0.235, 0.000, 0.000,
  0.353, 0.000, 0.000,
  0.471, 0.000, 0.000,
  0.588, 0.000, 0.000,
  0.706, 0.000, 0.000,
  0.824, 0.000, 0.000,
  0.941, 0.000, 0.000,
  1.000, 0.059, 0.059,
  1.000, 0.176, 0.176,
  1.000, 0.294, 0.294,
  1.000, 0.412, 0.412,
  1.000, 0.529, 0.529,
  1.000, 0.647, 0.647,
  1.000, 0.765, 0.765,
  1.000, 0.882, 0.882,
  0.000, 0.000, 0.000,
  0.235, 0.090, 0.000,
  0.353, 0.133, 0.000,
  0.471, 0.176, 0.000,
  0.588, 0.220, 0.000,
  0.706, 0.267, 0.000,
  0.824, 0.310, 0.000,
  0.941, 0.353, 0.000,
  1.000, 0.412, 0.059,
  1.000, 0.486, 0.176,
  1.000, 0.561, 0.294,
  1.000, 0.631, 0.412,
  1.000, 0.706, 0.529,
  1.000, 0.780, 0.647,
  1.000, 0.855, 0.765,
  1.000, 0.925, 0.882,
  0.071, 0.071, 0.071,
  0.235, 0.176, 0.000,
  0.353, 0.267, 0.000,
  0.471, 0.353, 0.000,
  0.588, 0.443, 0.000,
  0.706, 0.529, 0.000,
  0.824, 0.620, 0.000,
  0.941, 0.706, 0.000,
  1.000, 0.765, 0.059,
  1.000, 0.796, 0.176,
  1.000, 0.824, 0.294,
  1.000, 0.855, 0.412,
  1.000, 0.882, 0.529,
  1.000, 0.914, 0.647,
  1.000, 0.941, 0.765,
  1.000, 0.973, 0.882,
  0.141, 0.141, 0.141,
  0.208, 0.235, 0.000,
  0.310, 0.353, 0.000,
  0.412, 0.471, 0.000,
  0.514, 0.588, 0.000,
  0.620, 0.706, 0.000,
  0.722, 0.824, 0.000,
  0.824, 0.941, 0.000,
  0.882, 1.000, 0.059,
  0.898, 1.000, 0.176,
  0.914, 1.000, 0.294,
  0.925, 1.000, 0.412,
  0.941, 1.000, 0.529,
  0.957, 1.000, 0.647,
  0.973, 1.000, 0.765,
  0.984, 1.000, 0.882,
  0.216, 0.216, 0.216,
  0.118, 0.235, 0.000,
  0.176, 0.353, 0.000,
  0.235, 0.471, 0.000,
  0.294, 0.588, 0.000,
  0.353, 0.706, 0.000,
  0.412, 0.824, 0.000,
  0.471, 0.941, 0.000,
  0.529, 1.000, 0.059,
  0.588, 1.000, 0.176,
  0.647, 1.000, 0.294,
  0.706, 1.000, 0.412,
  0.765, 1.000, 0.529,
  0.824, 1.000, 0.647,
  0.882, 1.000, 0.765,
  0.941, 1.000, 0.882,
  0.286, 0.286, 0.286,
  0.031, 0.235, 0.000,
  0.043, 0.353, 0.000,
  0.059, 0.471, 0.000,
  0.075, 0.588, 0.000,
  0.090, 0.706, 0.000,
  0.102, 0.824, 0.000,
  0.118, 0.941, 0.000,
  0.176, 1.000, 0.059,
  0.278, 1.000, 0.176,
  0.384, 1.000, 0.294,
  0.486, 1.000, 0.412,
  0.588, 1.000, 0.529,
  0.690, 1.000, 0.647,
  0.796, 1.000, 0.765,
  0.898, 1.000, 0.882,
  0.357, 0.357, 0.357,
  0.000, 0.235, 0.059,
  0.000, 0.353, 0.090,
  0.000, 0.471, 0.118,
  0.000, 0.588, 0.149,
  0.000, 0.706, 0.176,
  0.000, 0.824, 0.208,
  0.000, 0.941, 0.235,
  0.059, 1.000, 0.294,
  0.176, 1.000, 0.384,
  0.294, 1.000, 0.471,
  0.412, 1.000, 0.561,
  0.529, 1.000, 0.647,
  0.647, 1.000, 0.737,
  0.765, 1.000, 0.824,
  0.882, 1.000, 0.914,
  0.427, 0.427, 0.427,
  0.000, 0.235, 0.149,
  0.000, 0.353, 0.220,
  0.000, 0.471, 0.294,
  0.000, 0.588, 0.369,
  0.000, 0.706, 0.443,
  0.000, 0.824, 0.514,
  0.000, 0.941, 0.588,
  0.059, 1.000, 0.647,
  0.176, 1.000, 0.690,
  0.294, 1.000, 0.737,
  0.412, 1.000, 0.780,
  0.529, 1.000, 0.824,
  0.647, 1.000, 0.867,
  0.765, 1.000, 0.914,
  0.882, 1.000, 0.957,
  0.502, 0.502, 0.502,
  0.000, 0.235, 0.235,
  0.000, 0.353, 0.353,
  0.000, 0.471, 0.471,
  0.000, 0.588, 0.588,
  0.000, 0.706, 0.706,
  0.000, 0.824, 0.824,
  0.000, 0.941, 0.941,
  0.059, 1.000, 1.000,
  0.176, 1.000, 1.000,
  0.294, 1.000, 1.000,
  0.412, 1.000, 1.000,
  0.529, 1.000, 1.000,
  0.647, 1.000, 1.000,
  0.765, 1.000, 1.000,
  0.882, 1.000, 1.000,
  0.573, 0.573, 0.573,
  0.000, 0.149, 0.235,
  0.000, 0.220, 0.353,
  0.000, 0.294, 0.471,
  0.000, 0.369, 0.588,
  0.000, 0.443, 0.706,
  0.000, 0.514, 0.824,
  0.000, 0.588, 0.941,
  0.059, 0.647, 1.000,
  0.176, 0.690, 1.000,
  0.294, 0.737, 1.000,
  0.412, 0.780, 1.000,
  0.529, 0.824, 1.000,
  0.647, 0.867, 1.000,
  0.765, 0.914, 1.000,
  0.882, 0.957, 1.000,
  0.643, 0.643, 0.643,
  0.000, 0.059, 0.235,
  0.000, 0.090, 0.353,
  0.000, 0.118, 0.471,
  0.000, 0.149, 0.588,
  0.000, 0.176, 0.706,
  0.000, 0.208, 0.824,
  0.000, 0.235, 0.941,
  0.059, 0.294, 1.000,
  0.176, 0.384, 1.000,
  0.294, 0.471, 1.000,
  0.412, 0.561, 1.000,
  0.529, 0.647, 1.000,
  0.647, 0.737, 1.000,
  0.765, 0.824, 1.000,
  0.882, 0.914, 1.000,
  0.714, 0.714, 0.714,
  0.031, 0.000, 0.235,
  0.043, 0.000, 0.353,
  0.059, 0.000, 0.471,
  0.075, 0.000, 0.588,
  0.090, 0.000, 0.706,
  0.102, 0.000, 0.824,
  0.118, 0.000, 0.941,
  0.176, 0.059, 1.000,
  0.278, 0.176, 1.000,
  0.384, 0.294, 1.000,
  0.486, 0.412, 1.000,
  0.588, 0.529, 1.000,
  0.690, 0.647, 1.000,
  0.796, 0.765, 1.000,
  0.898, 0.882, 1.000,
  0.784, 0.784, 0.784,
  0.118, 0.000, 0.235,
  0.176, 0.000, 0.353,
  0.235, 0.000, 0.471,
  0.294, 0.000, 0.588,
  0.353, 0.000, 0.706,
  0.412, 0.000, 0.824,
  0.471, 0.000, 0.941,
  0.529, 0.059, 1.000,
  0.588, 0.176, 1.000,
  0.647, 0.294, 1.000,
  0.706, 0.412, 1.000,
  0.765, 0.529, 1.000,
  0.824, 0.647, 1.000,
  0.882, 0.765, 1.000,
  0.941, 0.882, 1.000,
  0.859, 0.859, 0.859,
  0.208, 0.000, 0.235,
  0.310, 0.000, 0.353,
  0.412, 0.000, 0.471,
  0.514, 0.000, 0.588,
  0.620, 0.000, 0.706,
  0.722, 0.000, 0.824,
  0.824, 0.000, 0.941,
  0.882, 0.059, 1.000,
  0.898, 0.176, 1.000,
  0.914, 0.294, 1.000,
  0.925, 0.412, 1.000,
  0.941, 0.529, 1.000,
  0.957, 0.647, 1.000,
  0.973, 0.765, 1.000,
  0.984, 0.882, 1.000,
  0.929, 0.929, 0.929,
  0.235, 0.000, 0.176,
  0.353, 0.000, 0.267,
  0.471, 0.000, 0.353,
  0.588, 0.000, 0.443,
  0.706, 0.000, 0.529,
  0.824, 0.000, 0.620,
  0.941, 0.000, 0.706,
  1.000, 0.059, 0.765,
  1.000, 0.176, 0.796,
  1.000, 0.294, 0.824,
  1.000, 0.412, 0.855,
  1.000, 0.529, 0.882,
  1.000, 0.647, 0.914,
  1.000, 0.765, 0.941,
  1.000, 0.882, 0.973,
  1.000, 1.000, 1.000,
  0.235, 0.000, 0.090,
  0.353, 0.000, 0.133,
  0.471, 0.000, 0.176,
  0.588, 0.000, 0.220,
  0.706, 0.000, 0.267,
  0.824, 0.000, 0.310,
  0.941, 0.000, 0.353,
  1.000, 0.059, 0.412,
  1.000, 0.176, 0.486,
  1.000, 0.294, 0.561,
  1.000, 0.412, 0.631,
  1.000, 0.529, 0.706,
  1.000, 0.647, 0.780,
  1.000, 0.765, 0.855,
  1.000, 0.882, 0.925,
];