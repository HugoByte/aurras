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
import Table from "@mui/material/Table";
import TableBody from "@mui/material/TableBody";
import TableCell from "@mui/material/TableCell";
import TableContainer from "@mui/material/TableContainer";
import TableHead from "@mui/material/TableHead";
import TableRow from "@mui/material/TableRow";
import Paper from "@mui/material/Paper";

function Main() {
  const location = useLocation();
  const navigate = useNavigate();
  // const user_auth_token = location.state.auth_token;
  const [auther, setAuth] = useState(localStorage.getItem("authToken"));

  const [activeScreen, setActiveScreen] = useState("home", "table");

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

  const handleAbout = () => {
    activeScreen === "home"
  };
  const [inputText, setInputText] = useState("");
  const [outputText, setOutputText] = useState("");

  const handleInputChange = (e) => {
    setInputText(e.target.value);
  };

  const handleSubmit = (e) => {
    e.preventDefault();
    setOutputText(inputText);
    setInputText("");
  };

  const data = [
    {
      method: "Event1",
      from: "From",
      to: "To",
      hash: "Hash",
      timestamp: "Timestamp",
    },
    {
      method: "Event2",
      from: "From",
      to: "To",
      hash: "Hash",
      timestamp: "Timestamp",
    },
  ];

  function createData(method, from, to, hash, timestamp) {
    return { method, from, to, hash, timestamp };
  }

  const rows = [
    createData("method2", "from2", "to2", "hash2", "time2"),
    createData("method3", "from3", "to3", "hash3", "time3"),
  ];

  if (auther == "") {
    // console.log("please login");
    alert(" Please Login");
    navigate("/");
  }

  return (
    <>
      <Nav>
        <NavMenu>
          <NavLink to="/main" activeStyle OnClick={handleAbout}>
            Home
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

      {/* {activeScreen === "home" && ( */}
      <div className="one">
        <h1></h1>
        <p>
          Aurras is a middleware that acts as an event processor and a low code
          workflow orchestration platform. Aurras is being pitched as a
          next-generation system for enabling decentralized push notification.
          This middleware solution listens to events from blockchain
          applications and propagates them to a registered pool of MQTT brokers.
          The broader architecture consist of parachain from which the
          middleware listens for the events.
        </p>
        <p>
          Aurras is a middleware that acts as an event processor and a low code
          workflow orchestration platform. Aurras is being pitched as a
          next-generation system for enabling decentralized push notification.
          This middleware solution listens to events from blockchain
          applications and propagates them to a registered pool of MQTT brokers.
          The broader architecture consist of parachain from which the
          middleware listens for the events.
        </p>

        <p>
          Aurras is a middleware that acts as an event processor and a low code
          workflow orchestration platform. Aurras is being pitched as a
          next-generation system for enabling decentralized push notification.
          This middleware solution listens to events from blockchain
          applications and propagates them to a registered pool of MQTT brokers.
          The broader architecture consist of parachain from which the
          middleware listens for the events.
        </p>

        {activeScreen === "home" && (
          <div>
            {/* <center> <h1 style={{color: "rgb(29, 150, 90)"}}>Details</h1></center> */}
            <div className="source-url">
              <input
                type="text"
                placeholder="SOURCE_URL"
                value={inputText}
                onChange={handleInputChange}
              />
            </div>

            <div className="dst-url">
              <input
                type="text"
                placeholder="DESTINATION_URL"
                value={inputText}
                onChange={handleInputChange}
              />
            </div>
            <div className="address">
              <input
                type="text"
                placeholder="ADDRESS"
                value={inputText}
                onChange={handleInputChange}
              />
            </div>
            {/* <div className="submit-button">
              {activeScreen === "table" && <h1>hello</h1>}
              <button onClick={handleSubmit}>Submit</button>
            </div> */}
            {/* <div className="submit-button"> */}

            {/* <button
                onClick={() => {
                  handleChangeActiveScreen("table");
                }}
              >
                submit
              </button> */}
            {/* </div> */}
            <div className="submit-button">
              <button
                onClick={() => {
                  handleChangeActiveScreen("table");
                }}
              >
                submit
              </button>
            </div>
          </div>
        )}

        {activeScreen === "table" && (
          <div className="table-size">
            <h1></h1>

            <TableContainer component={Paper}>
              <Table sx={{ minWidth: 400 }} aria-label="simple table">
                <TableHead>
                  <TableRow>
                    <TableCell>
                      <b>Method</b>
                    </TableCell>
                    <TableCell align="right">
                      <b>From</b>
                    </TableCell>
                    <TableCell align="right">
                      <b>To&nbsp;</b>
                    </TableCell>
                    <TableCell align="right">
                      <b>Hash&nbsp;</b>
                    </TableCell>
                    <TableCell align="right">
                      <b>Timestamp&nbsp;</b>
                    </TableCell>
                  </TableRow>
                </TableHead>
                <TableBody>
                  {rows.map((row) => (
                    <TableRow
                      key={row.method}
                      sx={{
                        "&:last-child td, &:last-child th": { border: 0 },
                      }}
                    >
                      <TableCell component="th" scope="row">
                        {row.method}
                      </TableCell>
                      <TableCell align="right">{row.from}</TableCell>
                      <TableCell align="right">{row.to}</TableCell>
                      <TableCell align="right">{row.hash}</TableCell>
                      <TableCell align="right">{row.timestamp}</TableCell>
                    </TableRow>
                  ))}
                </TableBody>
              </Table>
            </TableContainer>
          </div>
        )}
      </div>

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
