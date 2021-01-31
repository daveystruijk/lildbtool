import "normalize.css";
import "@blueprintjs/icons/lib/css/blueprint-icons.css";
import '@blueprintjs/core/lib/css/blueprint.css';

import React, {useState} from 'react';
import ReactDOM from 'react-dom';
import { FocusStyleManager, Classes, Icon, Menu, MenuDivider, MenuItem, Navbar, Tab, Tabs, Alignment } from "@blueprintjs/core";
FocusStyleManager.onlyShowFocusOnTabs();

const App = () => {
  const [navbarTabId, setNavbarTabId] = useState("home");

  const handleNavbarTabChange = (navbarTabId) => {
    setNavbarTabId(navbarTabId);
  };

  return <div>
    <Tabs
      animate={true}
      large={true}
      vertical={true}
      onChange={handleNavbarTabChange}
      selectedTabId={navbarTabId}
    >
      <Tab id="home" title="Home" />
      <Tab id="files" title="Files" />
      <Tab id="builds" title="Builds" />
    </Tabs>
  </div>;
}

ReactDOM.render(<App/>, document.getElementById('app'));
