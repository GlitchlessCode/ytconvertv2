> [!WARNING]
> This repository is henceforth archived and will not be updated. Please be aware that it may be made private at any time.

<h1 align="center">YTConvert</h1>

<p align="center">
  <img alt="MacOS Badge" src="https://img.shields.io/badge/mac%20os-000000?style=for-the-badge&logo=apple&logoColor=white" />
  <img alt="Windows Badge" src="https://img.shields.io/badge/Windows-0078D6?style=for-the-badge&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiIHN0YW5kYWxvbmU9Im5vIj8+CjwhRE9DVFlQRSBzdmcgUFVCTElDICItLy9XM0MvL0RURCBTVkcgMS4xLy9FTiIgImh0dHA6Ly93d3cudzMub3JnL0dyYXBoaWNzL1NWRy8xLjEvRFREL3N2ZzExLmR0ZCI+Cjxzdmcgd2lkdGg9IjEwMCUiIGhlaWdodD0iMTAwJSIgdmlld0JveD0iMCAwIDI0IDI0IiB2ZXJzaW9uPSIxLjEiIHhtbG5zPSJodHRwOi8vd3d3LnczLm9yZy8yMDAwL3N2ZyIgeG1sbnM6eGxpbms9Imh0dHA6Ly93d3cudzMub3JnLzE5OTkveGxpbmsiIHhtbDpzcGFjZT0icHJlc2VydmUiIHhtbG5zOnNlcmlmPSJodHRwOi8vd3d3LnNlcmlmLmNvbS8iIHN0eWxlPSJmaWxsLXJ1bGU6ZXZlbm9kZDtjbGlwLXJ1bGU6ZXZlbm9kZDtzdHJva2UtbGluZWpvaW46cm91bmQ7c3Ryb2tlLW1pdGVybGltaXQ6MjsiPgogICAgPHBhdGggZD0iTTAsMEwxMS4zNzcsMEwxMS4zNzcsMTEuMzcyTDAsMTEuMzcyTDAsMFpNMTIuNjIzLDBMMjQsMEwyNCwxMS4zNzJMMTIuNjIzLDExLjM3MkwxMi42MjMsMFpNMCwxMi42MjNMMTEuMzc3LDEyLjYyM0wxMS4zNzcsMjRMMCwyNEwwLDEyLjYyM1pNMTIuNjIzLDEyLjYyM0wyNCwxMi42MjNMMjQsMjRMMTIuNjIzLDI0IiBzdHlsZT0iZmlsbDp3aGl0ZTtmaWxsLXJ1bGU6bm9uemVybzsiLz4KPC9zdmc+Cg==" />
  <img alt="Velopack Badge" src="https://img.shields.io/badge/Velopack-black?style=for-the-badge&logo=data:image/svg+xml;base64,PD94bWwgdmVyc2lvbj0iMS4wIiBlbmNvZGluZz0iVVRGLTgiIHN0YW5kYWxvbmU9Im5vIj8+CjwhRE9DVFlQRSBzdmcgUFVCTElDICItLy9XM0MvL0RURCBTVkcgMS4xLy9FTiIgImh0dHA6Ly93d3cudzMub3JnL0dyYXBoaWNzL1NWRy8xLjEvRFREL3N2ZzExLmR0ZCI+Cjxzdmcgd2lkdGg9IjEwMCUiIGhlaWdodD0iMTAwJSIgdmlld0JveD0iMCAwIDI1NiAyNTYiIHZlcnNpb249IjEuMSIgeG1sbnM9Imh0dHA6Ly93d3cudzMub3JnLzIwMDAvc3ZnIiB4bWxuczp4bGluaz0iaHR0cDovL3d3dy53My5vcmcvMTk5OS94bGluayIgeG1sOnNwYWNlPSJwcmVzZXJ2ZSIgeG1sbnM6c2VyaWY9Imh0dHA6Ly93d3cuc2VyaWYuY29tLyIgc3R5bGU9ImZpbGwtcnVsZTpldmVub2RkO2NsaXAtcnVsZTpldmVub2RkO3N0cm9rZS1saW5lam9pbjpyb3VuZDtzdHJva2UtbWl0ZXJsaW1pdDoyOyI+CiAgICA8ZyB0cmFuc2Zvcm09Im1hdHJpeCgxLjM4MTgxLDAsMCwxLjM4MTgxLDExLjM4OTksMzQuODA3NSkiPgogICAgICAgIDxnIGlkPSJMYXllcl8xLTIiPgogICAgICAgICAgICA8Zz4KICAgICAgICAgICAgICAgIDxwYXRoIGQ9Ik0xNjguNTgsMS45N0w4My4zOCwxMzQuM0M4Mi44OCwxMzUuMDggODEuNzMsMTM1LjA4IDgxLjIzLDEzNC4zTDUyLjU4LDg5Ljc5QzUyLjAzLDg4Ljk0IDUyLjY0LDg3LjgyIDUzLjY2LDg3LjgyTDEwMy4yMSw4Ny44MkMxMDQuNjEsODcuODIgMTA0Ljk5LDg1Ljg5IDEwMy42OSw4NS4zNkwzNi40Miw1Ny44MUMzNS4xMiw1Ny4yOCAzNS41LDU1LjM1IDM2LjksNTUuMzVMMTI0LjA2LDU1LjM1QzEyNS40Niw1NS4zNSAxMjUuODQsNTMuNDIgMTI0LjU0LDUyLjg5TDAuOCwyLjQ2Qy0wLjUsMS45MyAtMC4xMiwwIDEuMjgsMEwxNjcuNSwwQzE2OC41MSwwIDE2OS4xMiwxLjEyIDE2OC41NywxLjk3TDE2OC41OCwxLjk3WiIgc3R5bGU9ImZpbGw6dXJsKCNfTGluZWFyMSk7ZmlsbC1ydWxlOm5vbnplcm87Ii8+CiAgICAgICAgICAgIDwvZz4KICAgICAgICA8L2c+CiAgICA8L2c+CiAgICA8ZGVmcz4KICAgICAgICA8bGluZWFyR3JhZGllbnQgaWQ9Il9MaW5lYXIxIiB4MT0iMCIgeTE9IjAiIHgyPSIxIiB5Mj0iMCIgZ3JhZGllbnRVbml0cz0idXNlclNwYWNlT25Vc2UiIGdyYWRpZW50VHJhbnNmb3JtPSJtYXRyaXgoMTA1LjE4LC04Ni43MSw4Ni43MSwxMDUuMTgsMTYuMzgsODguMzkpIj48c3RvcCBvZmZzZXQ9IjAiIHN0eWxlPSJzdG9wLWNvbG9yOnJnYigyMjYsMTg5LDM1KTtzdG9wLW9wYWNpdHk6MSIvPjxzdG9wIG9mZnNldD0iMC45OSIgc3R5bGU9InN0b3AtY29sb3I6cmdiKDIzNywyMTEsMzUpO3N0b3Atb3BhY2l0eToxIi8+PHN0b3Agb2Zmc2V0PSIxIiBzdHlsZT0ic3RvcC1jb2xvcjpyZ2IoMjM3LDIxMSwzNSk7c3RvcC1vcGFjaXR5OjEiLz48L2xpbmVhckdyYWRpZW50PgogICAgPC9kZWZzPgo8L3N2Zz4K" />
  <img alt="Build Status" src="https://img.shields.io/github/actions/workflow/status/GlitchlessCode/ytconvertv2/buildrelease.yml?style=for-the-badge">
</p>

`ytconvertv2` is an app which provides a nice visual wrapper around [`yt-dlp`](https://github.com/yt-dlp/yt-dlp) and
[`ffmpeg`](https://ffmpeg.org/), in order to extract audio and video from YouTube videos in a variety of file formats.

## Table of Contents

- [Table of Contents](#table-of-contents)
- [Installation](#installation)
- [Support Me](#support-me)
- [Features](#features)
  - [Video Extraction](#video-extraction)
  - [Playlist Extraction](#playlist-extraction)
  - [Automatic Updates](#automatic-updates)
- [Improvements Over 1.x.x](#improvements-over-1xx)
  - [Task Queue](#task-queue)
  - [File Pre-Naming](#file-pre-naming)
  - [Automatic Dependency Updates](#automatic-dependency-updates)
  - [Better Error Handling](#better-error-handling)
- [License \& Legal Disclaimer](#license--legal-disclaimer)

## Installation

Download the installer for your platform from the table below and run it. Currently only x86_64 builds are produced,
and arm64 is unsupported.

_Note: For MacOS users, the first time you try to open the app, it may be quarantined by MacOS, because it's downloaded from the_
_internet. To open it, right-click the application and choose "Open"_

<table align="center">
<thead>
<tr>
<th align="center">Operating System</th>
<th align="center">Download</th>
</tr>
</thead>
<tbody>
<tr>
<td align="center">MacOS</td>
<td align="center"><a href="https://www.github.com/GlitchlessCode/ytconvertv2/releases/latest/download/ytconvertv2-osx-Setup.dmg">Download <code>ytconvertv2-osx-Setup.dmg</code></a></td>
</tr>
<tr>
<td align="center">Windows</td>
<td align="center"><a href="https://www.github.com/GlitchlessCode/ytconvertv2/releases/latest/download/ytconvertv2-win-Setup.exe">Download <code>ytconvertv2-win-Setup.exe</code></a></td>
</tr>
</tbody>
</table>

## Support Me

If you find this tool helpful, consider supporting future development:

[!["Buy Me A Coffee"](https://www.buymeacoffee.com/assets/img/custom_images/orange_img.png)](https://buymeacoffee.com/glitchlesscode)

## Features

### Video Extraction

`ytconvertv2` can extract any publicly available YouTube video in several audio and video formats! Is the
format you want missing? Recommend it [here](https://github.com/GlitchlessCode/ytconvertv2/issues/new?template=feature_request.yml)!

![Screenshot demo of video extraction](.assets/video-demo.png)

### Playlist Extraction

`ytconvertv2` can also extract entire playlists at a time! Just plug in the link, and let it run.

![Screenshot demo of playlist extraction](.assets/playlist-demo.png)

### Automatic Updates

Updates for `ytconvertv2` have never been easier! All dependencies are automatically fetched, and the app automatically
fetches the most recent version on startup. Powered by [Velopack](https://velopack.io/), `ytconvertv2` should no longer
require you to manually go download a new version ever again.

## Improvements Over 1.x.x

The 2.0.0 release of `ytconvertv2` comes with whole slew of major user experience improvements! Enjoy!

### Task Queue

Queue up multiple extraction tasks, both videos and playlists, in any format, and then just let `ytconvertv2` work in the
background.

### File Pre-Naming

All outputs can now be pre-named to a different name, should you so desire! Name anything and everything however you please.

### Automatic Dependency Updates

The rewrite of `ytconvertv2` now acts as a wrapper for [`yt-dlp`](https://github.com/yt-dlp/yt-dlp), and as such, can
update [`yt-dlp`](https://github.com/yt-dlp/yt-dlp) separately from `ytconvertv2`, creating a more resilient app
experience. The need for developer intervention to update dependencies should be drastically reduced, which helps both
you and me.

### Better Error Handling

Errors should almost never cause the program to crash anymore! A new error menu has been added, where errors can and
will be reported.

## License & Legal Disclaimer

This software is provided under the [MIT License](./LICENSE). However, the following additional terms and disclaimers
apply specifically to its use:

1. **Third-Party Dependencies**

   This software utilizes third-party tools and libraries including [`yt-dlp`](https://github.com/yt-dlp/yt-dlp),
   [`ffmpeg`](https://ffmpeg.org/), and various Rust crates for media extraction and conversion. These tools are
   included or referenced under their respective open-source licenses. The developer of this application is not
   affiliated with, nor responsible for, the development or maintenance of those projects.

2. **Intended Use**

   This application is intended solely as a general-purpose, **personal-use tool**. It does not facilitate or encourage
   the downloading of copyrighted or infringing content, nor is it designed with the intent to violate any intellectual
   property laws. Its primary purpose is to provide users with the ability to manage and convert media they have legal
   access to, including, but not limited to publicly available videos, self-created content, or content distributed under
   open licenses.

   The developer neither endorses nor assumes responsibility for the ways in which users utilize this tool. It is the
   user’s responsibility to ensure they possess the legal rights to download or convert any content. Use of this software
   for the unauthorized acquisition or redistribution of copyrighted material is **strictly discouraged**.

3. **Responsibility of Use**

   Users are solely responsible for how they choose to use this software. Downloading copyrighted content without
   appropriate rights or permission may violate the laws of your country and/or the Terms of Service of YouTube.

   The developer expressly disclaims any liability for actions taken by users in violation of such laws or terms.

4. **No Association with YouTube or Google**

   This application is not affiliated with, endorsed by, or connected to YouTube, Google, or any of their subsidiaries.
   Use of this tool may violate YouTube’s Terms of Service, and could result in actions against user accounts by those
   entities.

5. **Developer Affiliation And Stance**

   The developer is not affiliated with, nor does the developer endorse, any particular use case by end users. The
   developer does not publicly condone or encourage piracy or unlawful behavior, and no endorsement should be inferred
   from the availability of this software.

   Users are solely responsible for ensuring that their use of the application complies with all applicable laws and
   respects the rights of content owners. Any legal liability arising from misuse of this software rests entirely with
   the user.
