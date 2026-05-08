{ system }:

let
  pname = "koreader-companion";
  version = "0.0.1-alpha.1";
  releaseName = "${pname}-${version}";
  unsignedArtifactBasename = "${releaseName}-${system}-unsigned";
in
{
  inherit
    pname
    version
    releaseName
    system
    unsignedArtifactBasename
    ;

  unsignedArtifactFileName = "${unsignedArtifactBasename}.tar.gz";
}
