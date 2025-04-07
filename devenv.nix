{pkgs, ...}: {
  packages = with pkgs; [
    openssl
  ];

  languages = {
    rust.enable = true;
  };
}
