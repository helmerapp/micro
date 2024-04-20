![](banner.png)

# Helmer Micro

![Stage: Alpha](https://img.shields.io/static/v1?label=Stage&message=Alpha&color=2BB4AB)
[![License: AGPL3.0](https://img.shields.io/static/v1?label=Licence&message=AGPL%20v3&color=000)](https://www.gnu.org/licenses/agpl-3.0)
[![Twitter: @HelmerApp](https://img.shields.io/badge/Twitter-00acee?logo=twitter&logoColor=white)](https://twitter.com/helmerapp)
![Github Stars](https://img.shields.io/github/stars/helmerapp/micro)

Helmer Micro is a cross-platform GIF recording app — built using the awesome [Tauri](https://github.com/tauri-apps/tauri) framework, our high-performance capture engine [scap](https://github.com/helmerapp/scap) and the incredible GIF encoder, [Gifski](https://github.com/ImageOptim/gifski).

> ⚠️ This project is in Alpha and things break often. Please use at your own risk.

## Motivation

GIFs are clearly the internet's favourite file format, but notoriously hard to create. We wanted to create a small tool to allows easy and effortless creation of GIFs, and share it with everyone!

Alongside our small mission, this project also stands for two more things:
1. a demo project for our library [scap](https://github.com/helmerapp/scap) and
2. show that open-source apps can (and should) be beautifully designed!

## COSS?

Yup, Commercial Open-source Software!

Many of the libraries and tools we author are kept up-to-date and made available to everyone though our Github and other platforms. We believe in operating with full transparency, building alongside our community and supporting peer OSS projects along the way. This way, everyone benefits from each other's work instead of running circles within individual walled gardens.

If you benefit from our work, we request that you buy a license via [our website](https://www.helmer.app/micro). This commercialization allows us to become a sustainable business and eventually work towards a credible new way of funding our work and experiments!

Of course, you are more than welcome to build your own binaries using our source-code! Please reach out in the `#micro` channel on our Discord if you need help setting this up.

Lastly, don't forget to share your GIFs online with a `#MadeWithMicro` hashtag! We'd love to see and share what you make 🫶

## Development

### Pre-requisites

1. Set up all the [Tauri Pre-requisites](https://beta.tauri.app/guides/prerequisites/) to set up Rust and other system dependencies.
2. Install [NodeJS](https://nodejs.org/en) 20.0 (or newer)
3. Install Yarn by running `npm install --global yarn`

### Guide

This is a [Tauri](https://tauri.app) app written in Typescript and Rust. It uses [Astro](https://astro.build) for UI and Rust for the backend.

1. To spin up the dev environment, run `yarn tauri dev`
2. To build the app for your current OS/architecture, run `yarn tauri build`
3. Follow the [Conventional Commits](https://www.conventionalcommits.org/en/v1.0.0/#specification) spec to name branches and commits.

---

## Credits

(coming soon)
