{
  description = "A basic Rust devshell for Leptos with either stable or nightly toolchains";

  inputs = {
    nixpkgs.url      = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url  = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
          inherit system overlays;
        };

        # Common build inputs needed for Leptos development
        common_build_inputs = with pkgs; [
          openssl
          pkg-config
          cacert
          cargo-make
          trunk
          tailwindcss
          dart-sass
          cargo-leptos
          cargo-generate
          rust-analyzer
          leptosfmt
          wasm-bindgen-cli
        ];

        # Extra Darwin libraries, if needed
        darwin_frameworks = pkgs.lib.optionals pkgs.stdenv.isDarwin [
          pkgs.darwin.apple_sdk.frameworks.SystemConfiguration
        ];

        # Helper function to select stable or nightly
        rustToolchain = channel:
          if channel == "nightly"
          then pkgs.rust-bin.nightly (toolchain:
            toolchain.default.override {
              extensions = [ "rust-src" "rust-analyzer" ];
              targets = [ "wasm32-unknown-unknown" ];
            }
          )
          else pkgs.rust-bin.stable.latest.default.override {
            extensions = [ "rust-src" "rust-analyzer" ];
            targets = [ "wasm32-unknown-unknown" ];
          };

        # Function returning a devShell for a given channel
        mk_leptos_shell = channel: pkgs.mkShell {
          buildInputs = common_build_inputs ++ [
            (rustToolchain channel)
          ] ++ darwin_frameworks;

          shellHook = ''
            # Crab ASCII art
            echo "              *~^~^~*          "
            echo "        \\)  (  o o  )/      "
            echo "             /   -   \\         "
            echo "            \\ '-----' /       "
            echo "   🦀   Welcome to ${channel} Rust!   🦀 "
            echo ""
            echo "Happy hacking with Leptos!"
          '';
        };
      in
      {
        devShells = {
          default = mk_leptos_shell "stable";
          stable  = mk_leptos_shell "stable";
          nightly = mk_leptos_shell "nightly";
        };
      }
    );
}