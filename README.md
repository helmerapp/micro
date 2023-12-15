<p align="center">

<h1 align="center">Helmer Micro</h1>

<p align="center">
  <img src="https://img.shields.io/static/v1?label=Stage&message=Alpha&color=2BB4AB" />
  <a href="https://www.gnu.org/licenses/agpl-3.0">
    <img src="https://img.shields.io/static/v1?label=Licence&message=AGPL%20v3&color=000" />
  </a>
  <a href="https://twitter.com/helmerapp">
    <img src="https://img.shields.io/badge/Twitter-00acee?logo=twitter&logoColor=white" />
  </a>
  <img src="https://img.shields.io/github/stars/clearlysid/helmer-micro" />
</p>

  <p align="center">
A smol app to record GIFs from your desktop.<br>
<a href="https://www.helmer.app"><b>helmer.app/micro ✨</b></a><br><br>
</p>

> ⚠️ NOTE: This project is early in its development. New designs, features and technologies are being iterated on. Things can break often, please use at your own risk.

## Development

### Pre-requisites

1. Follow [this guide](https://tauri.app/v1/guides/getting-started/prerequisites) to set up Rust and other system dependencies.
2. Install [NodeJS](https://nodejs.org/en) v18.17.0 LTS (or newer)
3. Install Yarn Package Manger by running `npm install --global yarn`

### Guide

This is a [Tauri](https://tauri.app) app. It uses [Astro](https://astro.build) for UI and Rust for the backend.

1. To spin up the dev environment, run `yarn tauri dev`
2. To build the app for your current OS/architecture, run `yarn tauri build`
3. Follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/#specification) spec to name branches and commits.
