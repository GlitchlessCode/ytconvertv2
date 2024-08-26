const ffmpegPath = require("ffmpeg-static");

module.exports = {
  packagerConfig: {
    asar: true,
    icon: "./src/images/ytconvertv2_logo",
    extraResource: [ffmpegPath],
  },
  rebuildConfig: {},
  makers: [
    {
      name: "@electron-forge/maker-squirrel",
      config: {},
    },
    {
      name: "@electron-forge/maker-dmg",
    },
  ],
  plugins: [
    {
      name: "@electron-forge/plugin-auto-unpack-natives",
      config: {},
    },
  ],
};
