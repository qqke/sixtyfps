#!/usr/bin/env node
/* LICENSE BEGIN
    This file is part of the SixtyFPS Project -- https://sixtyfps.io
    Copyright (c) 2020 Olivier Goffart <olivier.goffart@sixtyfps.io>
    Copyright (c) 2020 Simon Hausmann <simon.hausmann@sixtyfps.io>

    SPDX-License-Identifier: GPL-3.0-only
    This file is also available under commercial licensing terms.
    Please contact info@sixtyfps.io for more information.
LICENSE END */

// import "sixtyfps";
require("sixtyfps");
// import * as demo from "../ui/printerdemo.60";
let demo = require("../ui/printerdemo.60");
let window = new demo.MainWindow();
window.show();

