Lapce plugin for checking crates.io for updates.

Note: this plugin only manages integration between [crates-lsp](https://github.com/MathiasPius/crates-lsp) and the Lapce editor.

For it to work, you *must* manually install the `crates-lsp` binary somewhere on your machine, and configure its path in the plugin settings!

# Usage

1. Install the `lapce-crates` plugin via your Lapce editor.
2. Download the latest `crates-lsp` release [here](https://github.com/MathiasPius/crates-lsp/releases).
3. Unpack the executable within somewhere on your machine. For example `/home/myname/.local/bin/crates-lsp`, if you're on linux.
4. In the Lapce editor settings, go to *Plugin Settings -> Crates* and set the *Lsp: Server Path* to the full path to the executable from step 3, e.g. `/home/myname/.local/bin/crates-lsp`

Once configured, you might have to reload the plugin.