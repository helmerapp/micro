# Helmer Micro

<p>
  <img src="https://img.shields.io/static/v1?label=Stage&message=Alpha&color=2BB4AB" />
  <a href="https://www.gnu.org/licenses/agpl-3.0">
    <img src="https://img.shields.io/static/v1?label=Licence&message=AGPL%20v3&color=000" />
  </a>
  <a href="https://twitter.com/helmerapp">
    <img src="https://img.shields.io/badge/Twitter-00acee?logo=twitter&logoColor=white" />
  </a>
  <img src="https://img.shields.io/github/stars/clearlysid/helmer-micro" />
</p>

> ⚠️ NOTE: This project is in early Alpha and things break often. Please use at your own risk.

## Installation

Helmer Micro is being developed as COSS: Commercial Open-source Software. It's source code is kept up-to-date and made available to everyone through this Github Repo. If you'd like to support our work, we request you to buy a license via [our website](https://www.helmer.app/micro) and share your GIFs online with a `#MadeWithMicro` hashtag that will allow us to find it online too!

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
