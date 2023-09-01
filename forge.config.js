module.exports = {
  packagerConfig: {
    asar: true,
    icon: "./src/images/ytconvertv2_logo",
    extraResource: ["./node_modules/ffmpeg-static/ffmpeg.exe"],
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
