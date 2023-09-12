import React from "react";

import { useLocation } from "react-router-dom";
import { Nav, NavLink, NavMenu } from "./nav";
import { useState } from "react";
import ModalApp from "./Modal";
import {
  Menu,
  Button,
  Dropdown,
  Container,
  Icon,
  Image,
  Label,
} from "semantic-ui-react";
import { useNavigate } from "react-router-dom";

function Main() {
  const location = useLocation();
  const navigate = useNavigate();
  // const user_auth_token = location.state.auth_token;
  const [auther, setAuth] = useState(localStorage.getItem("authToken"));

  const [activeScreen, setActiveScreen] = useState("home");

  function handleChangeActiveScreen(screen) {
    setActiveScreen(screen);
  }

  const handleLogout = () => {
    location.state.auth_token = "";
    console.log(location.state.auth_token);
    console.log("kaskkkf");
    setAuth("");
    localStorage.clear();
  };

  if (auther == "") {
    // console.log("please login");
    alert(" Please Login");
    navigate("/");
  }

  return (
    <>
      <Nav>
        <NavMenu>
          <NavLink to="/about" activeStyle>
            About
          </NavLink>
          {/* <NavLink to="/model" activeStyle>
            Register
          </NavLink> */}
          <Menu.Menu position="right" style={{ alignItems: "center" }}>
            {auther ? <ModalApp user_auth_token={auther}></ModalApp> : null}
            {!auther ? (
              <span>
                {/* Create an account with Polkadot-JS Extension (
              <a target="_blank" rel="noreferrer" href={CHROME_EXT_URL}>
                Chrome
              </a>
              ,&nbsp;
              <a target="_blank" rel="noreferrer" href={FIREFOX_ADDON_URL}>
                Firefox
              </a>
              )&nbsp; */}
              </span>
            ) : null}
          </Menu.Menu>

          <NavLink
            exact
            to={{ pathname: "/" }}
            activeStyle
            OnClick={handleLogout}
          >
            Sign Out
          </NavLink>
        </NavMenu>
      </Nav>
      <div className="division">
        <h1>Aurras</h1>
      </div>

      {activeScreen === "home" && (
        <div className="one">
          <h1></h1>
          <p>
            Aurras is a middleware that acts as an event processor and a low
            code workflow orchestration platform. Aurras is being pitched as a
            next-generation system for enabling decentralized push notification.
            This middleware solution listens to events from blockchain
            applications and propagates them to a registered pool of MQTT
            brokers. The broader architecture consist of parachain from which
            the middleware listens for the events.
          </p>
          <p>
            Aurras is a middleware that acts as an event processor and a low
            code workflow orchestration platform. Aurras is being pitched as a
            next-generation system for enabling decentralized push notification.
            This middleware solution listens to events from blockchain
            applications and propagates them to a registered pool of MQTT
            brokers. The broader architecture consist of parachain from which
            the middleware listens for the events.
          </p>

          <p>
            Aurras is a middleware that acts as an event processor and a low
            code workflow orchestration platform. Aurras is being pitched as a
            next-generation system for enabling decentralized push notification.
            This middleware solution listens to events from blockchain
            applications and propagates them to a registered pool of MQTT
            brokers. The broader architecture consist of parachain from which
            the middleware listens for the events.
          </p>
        </div>
      )}

      {activeScreen === "table" && <table>table</table>}

      <button
        onClick={() => {
          handleChangeActiveScreen("table");
        }}
      >
        Table
      </button>

      {/* <div className="two">
        <h1></h1>
      </div> */}

      <div className="three">
        <h1></h1>
        <p>Thanks to hugobyte</p>
      </div>
    </>
  );
}

export default Main;
