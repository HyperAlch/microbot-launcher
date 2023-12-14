let
  rust-overlay = builtins.fetchTarball "https://github.com/oxalica/rust-overlay/archive/master.tar.gz";
  pkgs = import <nixpkgs> {
    overlays = [(import rust-overlay)];
  };
  libraries = with pkgs;[
       webkitgtk
       gtk3
       cairo
       gdk-pixbuf
       glib
     
       dbus
       openssl_3
       librsvg
    ];
    packages = with pkgs; [
    	curl
    	wget
    	pkg-config
    	dbus
    	openssl_3
    	glib
    	gtk3
    	libsoup
    	webkitgtk
    	appimagekit
    	librsvg
      
    	
      rust-analyzer-unwrapped
      nodejs_20
      llvm
    ];
  toolchain = pkgs.rust-bin.fromRustupToolchainFile ./toolchain.toml;
in
  pkgs.clangStdenv.mkDerivation {
    name = "microbot-launcher-nix-shell";
    buildInputs = [
      toolchain
    ] ++ packages;
   
    LIBCLANG_PATH = "${pkgs.libclang.lib}/lib";
    shellHook =
      ''
        export LD_LIBRARY_PATH=${pkgs.lib.makeLibraryPath libraries}:$LD_LIBRARY_PATH
      '';
  }
  
  
