{ pkgs, metadata }:

let
  placeholderReadme = pkgs.writeText "unsigned-artifact-placeholder-readme.txt" ''
    Placeholder unsigned release artifact for ${metadata.pname}.

    This archive exists to preserve the future release-artifact boundary before
    the application package exists. Later changes can replace this placeholder
    payload without renaming the flake package output shape.

    Signing and publishing are intentionally out of scope for this artifact.
  '';

  releaseMetadataJson = pkgs.writeText "release-metadata.json" (builtins.toJSON metadata);
in
pkgs.runCommand metadata.unsignedArtifactFileName
  {
    nativeBuildInputs = with pkgs; [
      gnutar
      gzip
    ];
    meta = {
      description = "Placeholder unsigned release artifact";
    };
    passthru = {
      releaseMetadata = metadata;
    };
  }
  ''
    artifact_dir="$TMPDIR/${metadata.unsignedArtifactBasename}"

    mkdir -p "$artifact_dir"
    cp ${placeholderReadme} "$artifact_dir/README.txt"
    cp ${releaseMetadataJson} "$artifact_dir/release-metadata.json"

    chmod 755 "$artifact_dir"
    chmod 444 "$artifact_dir/README.txt" "$artifact_dir/release-metadata.json"

    export LC_ALL=C

    tar \
      --sort=name \
      --mtime='@1' \
      --owner=0 \
      --group=0 \
      --numeric-owner \
      -C "$TMPDIR" \
      -cf - \
      "${metadata.unsignedArtifactBasename}" \
      | gzip -n > "$out"
  ''
