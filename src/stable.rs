#![cfg_attr(rustfmt, rustfmt_skip)]

pub mod gamescope_pipewire {
    //! Gamescope Pipewire Protocol
    //!
    //! Allows interaction with gamescope pipewire protocol
    wayland_protocol!(
        "gamescope-pipewire",
        [],
        []
    );
}

pub mod gamescope_input_method {
    //! Gamescope Input Method Protocol
    //!
    //! Allows interaction with gamescope input protocol
    wayland_protocol!(
        "gamescope-input-method",
        [
            (wl_seat, wl_seat_interface), 
            (wl_surface, wl_surface_interface)
        ],
        []
    );
}

pub mod gamescope_xwayland {
    //! Gamescope XWayland Protocol
    //!
    //! Allows interaction with gamescope xwayland protocol
    wayland_protocol!(
        "gamescope-xwayland",
        [
            (wl_surface, wl_surface_interface)
        ],
        []
    );
}