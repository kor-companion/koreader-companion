{ pkgs }:

let
  metadata = import ../release/metadata.nix {
    system = pkgs.stdenv.hostPlatform.system;
  };
  unsignedReleaseArtifact = import ./unsigned-release-artifact.nix {
    inherit metadata pkgs;
  };
in
{
  default = unsignedReleaseArtifact;
  "unsigned-release-artifact" = unsignedReleaseArtifact;
}
