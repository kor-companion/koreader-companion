{ pkgs }:

let
  metadata = import ./release/metadata.nix {
    system = pkgs.stdenv.hostPlatform.system;
  };
  packages = import ./packages { inherit pkgs; };
  releaseMetadataJson = pkgs.writeText "release-metadata.json" (builtins.toJSON metadata);
  unsignedReleaseArtifact = packages."unsigned-release-artifact";
  sourceSizeCheck = pkgs.runCommand "source-size-check"
    {
      nativeBuildInputs = [ pkgs.python3 ];
    }
    ''
      cd ${../.}
      python3 ${../scripts/check-source-size.py}
      mkdir -p "$out"
      touch "$out/verified"
    '';
in
{
  "source-size" = sourceSizeCheck;

  "release-metadata" = pkgs.runCommand "release-metadata-check"
    { }
    ''
      case "${metadata.unsignedArtifactFileName}" in
        *-unsigned.tar.gz) ;;
        *)
          printf '%s\n' 'expected an unsigned artifact filename ending in -unsigned.tar.gz' >&2
          exit 1
          ;;
      esac

      mkdir -p "$out"
      cp ${releaseMetadataJson} "$out/release-metadata.json"
    '';

  "unsigned-release-artifact" = pkgs.runCommand "unsigned-release-artifact-check"
    {
      nativeBuildInputs = with pkgs; [
        gnutar
        jq
      ];
    }
    ''
      artifact_dir="$TMPDIR/artifact"

      mkdir -p "$artifact_dir"
      tar -xzf ${unsignedReleaseArtifact} -C "$artifact_dir"

      test -f "$artifact_dir/${metadata.unsignedArtifactBasename}/README.txt"
      test -f "$artifact_dir/${metadata.unsignedArtifactBasename}/release-metadata.json"

      jq -e \
        '.unsignedArtifactFileName == "${metadata.unsignedArtifactFileName}" and .system == "${metadata.system}"' \
        "$artifact_dir/${metadata.unsignedArtifactBasename}/release-metadata.json" > /dev/null

      mkdir -p "$out"
      touch "$out/verified"
    '';
}
