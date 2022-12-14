# This file was @generated by cargo2nix 0.11.0.
# It is not intended to be manually edited.

args@{
  release ? true,
  rootFeatures ? [
    "eipher/default"
  ],
  rustPackages,
  buildRustPackages,
  hostPlatform,
  hostPlatformCpu ? null,
  hostPlatformFeatures ? [],
  target ? null,
  codegenOpts ? null,
  profileOpts ? null,
  rustcLinkFlags ? null,
  rustcBuildFlags ? null,
  mkRustCrate,
  rustLib,
  lib,
  workspaceSrc,
}:
let
  workspaceSrc = if args.workspaceSrc == null then ./. else args.workspaceSrc;
in let
  inherit (rustLib) fetchCratesIo fetchCrateLocal fetchCrateGit fetchCrateAlternativeRegistry expandFeatures decideProfile genDrvsByProfile;
  profilesByName = {
  };
  rootFeatures' = expandFeatures rootFeatures;
  overridableMkRustCrate = f:
    let
      drvs = genDrvsByProfile profilesByName ({ profile, profileName }: mkRustCrate ({ inherit release profile hostPlatformCpu hostPlatformFeatures target profileOpts codegenOpts rustcLinkFlags rustcBuildFlags; } // (f profileName)));
    in { compileMode ? null, profileName ? decideProfile compileMode release }:
      let drv = drvs.${profileName}; in if compileMode == null then drv else drv.override { inherit compileMode; };
in
{
  cargo2nixVersion = "0.11.0";
  workspace = {
    eipher = rustPackages.unknown.eipher."0.1.0";
  };
  "registry+https://github.com/rust-lang/crates.io-index".aho-corasick."0.7.19" = overridableMkRustCrate (profileName: rec {
    name = "aho-corasick";
    version = "0.7.19";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "b4f55bd91a0978cbfd91c457a164bab8b4001c833b7f323132c0a4e1922dd44e"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "std" ]
    ];
    dependencies = {
      memchr = rustPackages."registry+https://github.com/rust-lang/crates.io-index".memchr."2.5.0" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".atty."0.2.14" = overridableMkRustCrate (profileName: rec {
    name = "atty";
    version = "0.2.14";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "d9b39be18770d11421cdb1b9947a45dd3f37e93092cbf377614828a319d5fee8"; };
    dependencies = {
      ${ if hostPlatform.parsed.kernel.name == "hermit" then "hermit_abi" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".hermit-abi."0.1.19" { inherit profileName; };
      ${ if hostPlatform.isUnix then "libc" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".libc."0.2.135" { inherit profileName; };
      ${ if hostPlatform.isWindows then "winapi" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".winapi."0.3.9" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".autocfg."1.1.0" = overridableMkRustCrate (profileName: rec {
    name = "autocfg";
    version = "1.1.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "d468802bab17cbc0cc575e9b053f41e72aa36bfa6b7f55e3529ffa43161b97fa"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".bitflags."1.3.2" = overridableMkRustCrate (profileName: rec {
    name = "bitflags";
    version = "1.3.2";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "bef38d45163c2f1dde094a7dfd33ccf595c92905c8f8f4fdc18d06fb1037718a"; };
    features = builtins.concatLists [
      [ "default" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".cc."1.0.73" = overridableMkRustCrate (profileName: rec {
    name = "cc";
    version = "1.0.73";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "2fff2a6927b3bb87f9595d67196a70493f627687a71d87a0d692242c33f58c11"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".cfg-if."1.0.0" = overridableMkRustCrate (profileName: rec {
    name = "cfg-if";
    version = "1.0.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "baf1de4339761588bc0619e3cbc0120ee582ebb74b53b4efbf79117bd2da40fd"; };
  });
  
  "unknown".eipher."0.1.0" = overridableMkRustCrate (profileName: rec {
    name = "eipher";
    version = "0.1.0";
    registry = "unknown";
    src = fetchCrateLocal workspaceSrc;
    dependencies = {
      env_logger = rustPackages."registry+https://github.com/rust-lang/crates.io-index".env_logger."0.9.1" { inherit profileName; };
      io_uring = rustPackages."registry+https://github.com/rust-lang/crates.io-index".io-uring."0.5.7" { inherit profileName; };
      libc = rustPackages."registry+https://github.com/rust-lang/crates.io-index".libc."0.2.135" { inherit profileName; };
      log = rustPackages."registry+https://github.com/rust-lang/crates.io-index".log."0.4.17" { inherit profileName; };
      nix = rustPackages."registry+https://github.com/rust-lang/crates.io-index".nix."0.25.0" { inherit profileName; };
      syscalls = rustPackages."registry+https://github.com/rust-lang/crates.io-index".syscalls."0.6.7" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".env_logger."0.9.1" = overridableMkRustCrate (profileName: rec {
    name = "env_logger";
    version = "0.9.1";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "c90bf5f19754d10198ccb95b70664fc925bd1fc090a0fd9a6ebc54acc8cd6272"; };
    features = builtins.concatLists [
      [ "atty" ]
      [ "default" ]
      [ "humantime" ]
      [ "regex" ]
      [ "termcolor" ]
    ];
    dependencies = {
      atty = rustPackages."registry+https://github.com/rust-lang/crates.io-index".atty."0.2.14" { inherit profileName; };
      humantime = rustPackages."registry+https://github.com/rust-lang/crates.io-index".humantime."2.1.0" { inherit profileName; };
      log = rustPackages."registry+https://github.com/rust-lang/crates.io-index".log."0.4.17" { inherit profileName; };
      regex = rustPackages."registry+https://github.com/rust-lang/crates.io-index".regex."1.6.0" { inherit profileName; };
      termcolor = rustPackages."registry+https://github.com/rust-lang/crates.io-index".termcolor."1.1.3" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".hermit-abi."0.1.19" = overridableMkRustCrate (profileName: rec {
    name = "hermit-abi";
    version = "0.1.19";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "62b467343b94ba476dcb2500d242dadbb39557df889310ac77c5d99100aaac33"; };
    features = builtins.concatLists [
      [ "default" ]
    ];
    dependencies = {
      libc = rustPackages."registry+https://github.com/rust-lang/crates.io-index".libc."0.2.135" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".humantime."2.1.0" = overridableMkRustCrate (profileName: rec {
    name = "humantime";
    version = "2.1.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "9a3a5bfb195931eeb336b2a7b4d761daec841b97f947d34394601737a7bba5e4"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".io-uring."0.5.7" = overridableMkRustCrate (profileName: rec {
    name = "io-uring";
    version = "0.5.7";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "6d32c9c053ad47572e11da8bce622ed4c9ae9dedac5b7f678a2e876d1494d4c4"; };
    dependencies = {
      bitflags = rustPackages."registry+https://github.com/rust-lang/crates.io-index".bitflags."1.3.2" { inherit profileName; };
      libc = rustPackages."registry+https://github.com/rust-lang/crates.io-index".libc."0.2.135" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".libc."0.2.135" = overridableMkRustCrate (profileName: rec {
    name = "libc";
    version = "0.2.135";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "68783febc7782c6c5cb401fbda4de5a9898be1762314da0bb2c10ced61f18b0c"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "extra_traits" ]
      [ "std" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".log."0.4.17" = overridableMkRustCrate (profileName: rec {
    name = "log";
    version = "0.4.17";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "abb12e687cfb44aa40f41fc3978ef76448f9b6038cad6aef4259d3c095a2382e"; };
    features = builtins.concatLists [
      [ "std" ]
    ];
    dependencies = {
      cfg_if = rustPackages."registry+https://github.com/rust-lang/crates.io-index".cfg-if."1.0.0" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".memchr."2.5.0" = overridableMkRustCrate (profileName: rec {
    name = "memchr";
    version = "2.5.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "2dffe52ecf27772e601905b7522cb4ef790d2cc203488bbd0e2fe85fcb74566d"; };
    features = builtins.concatLists [
      [ "default" ]
      [ "std" ]
    ];
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".memoffset."0.6.5" = overridableMkRustCrate (profileName: rec {
    name = "memoffset";
    version = "0.6.5";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "5aa361d4faea93603064a027415f07bd8e1d5c88c9fbf68bf56a285428fd79ce"; };
    features = builtins.concatLists [
      [ "default" ]
    ];
    buildDependencies = {
      autocfg = buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".autocfg."1.1.0" { profileName = "__noProfile"; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".nix."0.25.0" = overridableMkRustCrate (profileName: rec {
    name = "nix";
    version = "0.25.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "e322c04a9e3440c327fca7b6c8a63e6890a32fa2ad689db972425f07e0d22abb"; };
    features = builtins.concatLists [
      [ "acct" ]
      [ "aio" ]
      [ "default" ]
      [ "dir" ]
      [ "env" ]
      [ "event" ]
      [ "feature" ]
      [ "fs" ]
      [ "hostname" ]
      [ "inotify" ]
      [ "ioctl" ]
      [ "kmod" ]
      [ "memoffset" ]
      [ "mman" ]
      [ "mount" ]
      [ "mqueue" ]
      [ "net" ]
      [ "personality" ]
      [ "pin-utils" ]
      [ "poll" ]
      [ "process" ]
      [ "pthread" ]
      [ "ptrace" ]
      [ "quota" ]
      [ "reboot" ]
      [ "resource" ]
      [ "sched" ]
      [ "signal" ]
      [ "socket" ]
      [ "term" ]
      [ "time" ]
      [ "ucontext" ]
      [ "uio" ]
      [ "user" ]
      [ "zerocopy" ]
    ];
    dependencies = {
      bitflags = rustPackages."registry+https://github.com/rust-lang/crates.io-index".bitflags."1.3.2" { inherit profileName; };
      cfg_if = rustPackages."registry+https://github.com/rust-lang/crates.io-index".cfg-if."1.0.0" { inherit profileName; };
      libc = rustPackages."registry+https://github.com/rust-lang/crates.io-index".libc."0.2.135" { inherit profileName; };
      ${ if !(hostPlatform.parsed.kernel.name == "redox") then "memoffset" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".memoffset."0.6.5" { inherit profileName; };
      pin_utils = rustPackages."registry+https://github.com/rust-lang/crates.io-index".pin-utils."0.1.0" { inherit profileName; };
    };
    buildDependencies = {
      autocfg = buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".autocfg."1.1.0" { profileName = "__noProfile"; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".pin-utils."0.1.0" = overridableMkRustCrate (profileName: rec {
    name = "pin-utils";
    version = "0.1.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "8b870d8c151b6f2fb93e84a13146138f05d02ed11c7e7c54f8826aaaf7c9f184"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".regex."1.6.0" = overridableMkRustCrate (profileName: rec {
    name = "regex";
    version = "1.6.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "4c4eb3267174b8c6c2f654116623910a0fef09c4753f8dd83db29c48a0df988b"; };
    features = builtins.concatLists [
      [ "aho-corasick" ]
      [ "memchr" ]
      [ "perf" ]
      [ "perf-cache" ]
      [ "perf-dfa" ]
      [ "perf-inline" ]
      [ "perf-literal" ]
      [ "std" ]
    ];
    dependencies = {
      aho_corasick = rustPackages."registry+https://github.com/rust-lang/crates.io-index".aho-corasick."0.7.19" { inherit profileName; };
      memchr = rustPackages."registry+https://github.com/rust-lang/crates.io-index".memchr."2.5.0" { inherit profileName; };
      regex_syntax = rustPackages."registry+https://github.com/rust-lang/crates.io-index".regex-syntax."0.6.27" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".regex-syntax."0.6.27" = overridableMkRustCrate (profileName: rec {
    name = "regex-syntax";
    version = "0.6.27";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "a3f87b73ce11b1619a3c6332f45341e0047173771e8b8b73f87bfeefb7b56244"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".syscalls."0.6.7" = overridableMkRustCrate (profileName: rec {
    name = "syscalls";
    version = "0.6.7";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "535e4a480d47370482f8251117cba053e32067862c439dcd4c9ea4026d08f88e"; };
    buildDependencies = {
      cc = buildRustPackages."registry+https://github.com/rust-lang/crates.io-index".cc."1.0.73" { profileName = "__noProfile"; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".termcolor."1.1.3" = overridableMkRustCrate (profileName: rec {
    name = "termcolor";
    version = "1.1.3";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "bab24d30b911b2376f3a13cc2cd443142f0c81dda04c118693e35b3835757755"; };
    dependencies = {
      ${ if hostPlatform.isWindows then "winapi_util" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".winapi-util."0.1.5" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".winapi."0.3.9" = overridableMkRustCrate (profileName: rec {
    name = "winapi";
    version = "0.3.9";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "5c839a674fcd7a98952e593242ea400abe93992746761e38641405d28b00f419"; };
    features = builtins.concatLists [
      [ "consoleapi" ]
      [ "errhandlingapi" ]
      [ "fileapi" ]
      [ "minwinbase" ]
      [ "minwindef" ]
      [ "processenv" ]
      [ "std" ]
      [ "winbase" ]
      [ "wincon" ]
      [ "winerror" ]
      [ "winnt" ]
    ];
    dependencies = {
      ${ if hostPlatform.config == "i686-pc-windows-gnu" then "winapi_i686_pc_windows_gnu" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".winapi-i686-pc-windows-gnu."0.4.0" { inherit profileName; };
      ${ if hostPlatform.config == "x86_64-pc-windows-gnu" then "winapi_x86_64_pc_windows_gnu" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".winapi-x86_64-pc-windows-gnu."0.4.0" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".winapi-i686-pc-windows-gnu."0.4.0" = overridableMkRustCrate (profileName: rec {
    name = "winapi-i686-pc-windows-gnu";
    version = "0.4.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "ac3b87c63620426dd9b991e5ce0329eff545bccbbb34f3be09ff6fb6ab51b7b6"; };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".winapi-util."0.1.5" = overridableMkRustCrate (profileName: rec {
    name = "winapi-util";
    version = "0.1.5";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "70ec6ce85bb158151cae5e5c87f95a8e97d2c0c4b001223f33a334e3ce5de178"; };
    dependencies = {
      ${ if hostPlatform.isWindows then "winapi" else null } = rustPackages."registry+https://github.com/rust-lang/crates.io-index".winapi."0.3.9" { inherit profileName; };
    };
  });
  
  "registry+https://github.com/rust-lang/crates.io-index".winapi-x86_64-pc-windows-gnu."0.4.0" = overridableMkRustCrate (profileName: rec {
    name = "winapi-x86_64-pc-windows-gnu";
    version = "0.4.0";
    registry = "registry+https://github.com/rust-lang/crates.io-index";
    src = fetchCratesIo { inherit name version; sha256 = "712e227841d057c1ee1cd2fb22fa7e5a5461ae8e48fa2ca79ec42cfc1931183f"; };
  });
  
}
