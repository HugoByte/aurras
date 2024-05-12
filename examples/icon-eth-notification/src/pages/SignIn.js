import React from "react";
import config from "../config";
import { useNavigate } from "react-router-dom";

function SignInForm() {
  const navigate = useNavigate();
  const [state, setState] = React.useState({
    email: "",
    password: "",
  });

  const [auth_token, setAuths] = React.useState("");

  const handleChange = (evt) => {
    const value = evt.target.value;
    setState({
      ...state,
      [evt.target.name]: value,
    });
  };

  const handleOnSubmit = (evt) => {
    evt.preventDefault();

    const { email, password } = state;

    var requestOptions = {
      method: "POST",
      headers: {
        "Content-Type": "application/json",
      },
      body: JSON.stringify({ email, password }),
    };
    console.log(requestOptions);
    fetch(`${config.api}default/user-login.json`, requestOptions)
      .then((response) => response.json())
      .then((data) => {
        console.log(data["error"]);
        if (data.error != undefined) {
          alert("Invalid username or password please login again");
          navigate("/");
        } else {
          const asd = data;
          const user = asd["user_token"];
          localStorage.setItem("authToken", user);
          navigate("/main");
        }
      })
      .catch((err) => {
        alert(`Error ${err}`);
        console.log(err);
      });

    for (const key in state) {
      setState({
        ...state,
        [key]: "",
      });
    }
  };

  return (
    <div className="form-container sign-in-container">
      <form onSubmit={handleOnSubmit}>
        <h1>Sign in</h1>
        <input
          type="email"
          placeholder="Email"
          name="email"
          value={state.email}
          onChange={handleChange}
        />
        <input
          type="password"
          name="password"
          placeholder="Password"
          value={state.password}
          onChange={handleChange}
        />
        <a href="#">Forgot your password?</a>
        <button>Sign In</button>
      </form>
    </div>
  );
}

export default SignInForm;
