{
  description = "A Rust development environment for midge, a no_std Rust MQTT client library.";

  # inputs are the dependencies of the flake
  inputs = {
    # Specify where nixpkgs come from
    nixpkgs.url = "github:nixos/nixpkgs?ref=nixos-24.11";
  };

  # outputs are the actual things that the flake produces
  outputs = { self, nixpkgs }: 
    let
      # List of supported architectures
      supportedSystems = ["x86_64-linux" "aarch64-darwin"];

      # Function to produce a development shell for any given system
      mkDevShell = system:
        let 
          pkgs = import nixpkgs {system = system;};

          rustVersion = "1.84.0";
        in pkgs.mkShell {
          # Establish the required binaries and libraries for embedded Rust development
          buildInputs = [
            pkgs.rustup # Rust version manager
            pkgs.espup # ESP development bootstrapper
          ];

          # Setup the shell environment for embedded Rust development;
          # Install the Rust toolchain, the embedded target, and set the linker 
          shellHook = ''
            # Set up Rust language server
            if ! rustup component list --installed | grep -q "rust-analyzer"; then
              echo "🔧 Installing rust-analyzer...";
              rustup component add rust-analyzer
            fi

            # Set up Rust formatter
            if ! rustup component list --installed | grep -q "rustfmt"; then
              echo "🔧 Installing rustfmt...";
              rustup component add rustfmt
            fi

            echo "🐚 Rust environment ready! 🦀"
          '';
        };

      # helper function to get nixpkgs for a specific platform
      # maps over provided systems, returning a list of attributes mapping the name to its packages value
      forAllSystems = systems: f: builtins.listToAttrs (map (system: {
        name = system;
        value = f system;
      }) systems);

    in {
      # Define the development shell for each supported system
      devShell = forAllSystems supportedSystems mkDevShell;
  };
}
