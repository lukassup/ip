use clap::{Parser, Subcommand};

/// ip command for mac, inspired by iproute2
#[derive(Parser)]
#[clap(infer_subcommands = true)]
#[command(version, about, long_about = None)]
pub(crate) enum IP {
    #[command(subcommand)]
    Address(Address),

    #[command(subcommand)]
    Link(Link),

    #[clap(alias = "neighbour")]
    #[command(subcommand)]
    Neighbor(Neighbor),

    #[command(subcommand)]
    Route(Route),
}

/// ip address { show | add | change | replace | del | save | flush | showdump | restore }
#[derive(Subcommand, Clone, Copy, Debug)]
pub(crate) enum Address {
    /// ip address show
    Show,
    /// TODO
    Add,
    /// TODO
    Change,
    /// TODO
    Replace,
    /// TODO
    Del,
    /// TODO
    Save,
    /// TODO
    Flush,
    /// TODO
    Showdump,
    /// TODO
    Restore,
}

/// ip link { show | add | delete | set | xstats | afstats | property-add | property-del }
#[derive(Subcommand, Clone, Copy, Debug)]
pub(crate) enum Link {
    /// ip link show [ DEVICE | group GROUP ] [up] [master DEV] [vrf NAME] [type TYPE]
    Show,
    /// TODO
    Add,
    /// TODO
    Delete,
    /// TODO
    Set,
    /// TODO
    Xstats,
    /// TODO
    Afstats,
    /// TODO
    PropertyAdd,
    /// TODO
    PropertyDel,
}

/// ip route { list | get | flush | save | restore | showdump }
#[derive(Subcommand, Clone, Copy, Debug)]
pub(crate) enum Route {
    /// TODO
    List,
    /// TODO
    Get,
    /// TODO
    Flush,
    /// TODO
    Save,
    /// TODO
    Restore,
    /// TODO
    Showdump,
}

/// ip { neighbor | neighbour } { show | get | add | del | change | replace | flush }
#[derive(Subcommand, Clone, Copy, Debug)]
pub(crate) enum Neighbor {
    /// TODO
    Show,
    /// TODO
    Get,
    /// TODO
    Add,
    /// TODO
    Del,
    /// TODO
    Change,
    /// TODO
    Replace,
    /// TODO
    Flush,
}
