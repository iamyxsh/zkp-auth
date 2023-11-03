/* eslint-disable jsx-a11y/img-redundant-alt */
/* eslint-disable jsx-a11y/anchor-is-valid */
import React, { useEffect, useReducer, useState } from 'react';
import init, * as wasm from './pkg/zkp_client';
import axios from 'axios';

const userState = {
  username: 'yash',
  password: 'password',
};

const msgState = {
  success: '',
  error: '',
};

const reducer = (state: typeof userState, action: { type: "SET_USERNAME" | "SET_PASSWORD", payload: string }) => {
  switch (action.type) {
    case 'SET_USERNAME':
      return { ...state, username: action.payload };
    case 'SET_PASSWORD':
      return { ...state, password: action.payload };
    default:
      return state;
  }
};

const msgReducer = (state: typeof msgState, action: { type: "SET_SUCCESS" | "SET_ERROR", payload: string }) => {
  switch (action.type) {
    case 'SET_SUCCESS':
      return { ...state, success: action.payload };
    case 'SET_ERROR':
      return { ...state, error: action.payload };
    default:
      return state;
  }
};



function App() {
  const [state, dispatch] = useReducer(reducer, userState);
  const [action, setAction] = useState<"Login" | "Register">("Login");
  const [msg, msgDispatch] = useReducer(msgReducer, msgState);

  const handleUsernameChange = (e: { target: { value: any; }; }) => {
    dispatch({ type: 'SET_USERNAME', payload: e.target.value });
  };

  const handlePasswordChange = (e: { target: { value: any; }; }) => {
    dispatch({ type: 'SET_PASSWORD', payload: e.target.value });
  };

  const handleRegister = async () => {

    const { y1, y2 } = JSON.parse(wasm.generate_y(state.password))
    const headers = new Headers()
    headers.append("Content-Type", "application/json")
    const res = await fetch("http://localhost:5000/api/register", {
      method: "POST",
      headers,
      body: JSON.stringify({
        y1, y2, username: state.username
      })
    }).then(res => res.json()).catch(err => msgDispatch({ type: "SET_ERROR", payload: err.msg }))

    msgDispatch({ type: "SET_SUCCESS", payload: res.msg })
  }

  const handleLogin = async () => {
    const k = wasm.gen_random_below()
    const { r1, r2 } = JSON.parse(wasm.generate_r(k))

    const headers = new Headers()
    headers.append("Content-Type", "application/json")
    const { c, auth_id } = await fetch("http://localhost:5000/api/challenge", {
      method: "POST",
      headers,
      body: JSON.stringify({
        username: state.username, r1, r2
      })
    }).then(res => res.json()).catch(err => msgDispatch({ type: "SET_ERROR", payload: err.msg }))

    let { solve } = JSON.parse(wasm.find_solve(k, c, state.password))

    const res = await fetch("http://localhost:5000/api/verify", {
      method: "POST",
      headers,
      body: JSON.stringify({
        username: state.username, s: solve, auth_id
      })
    }).then(res => res.json()).catch(err => msgDispatch({ type: "SET_ERROR", payload: err.msg }))

    msgDispatch({ type: "SET_SUCCESS", payload: res.msg })
  }

  useEffect(() => {
    const initWasm = async () => await init()
    initWasm()
  }, [])

  useEffect(() => {
    if (msg.success !== "" || msg.success !== "") {
      setTimeout(() => {
        msgDispatch({ type: "SET_ERROR", payload: "" })
        msgDispatch({ type: "SET_SUCCESS", payload: "" })
      }, 5000)
    }
  }, [msg])

  return (
    <section className="h-screen flex flex-col md:flex-row justify-center space-y-10 md:space-y-0 md:space-x-16 items-center my-2 mx-5 md:mx-0 md:my-0">
      <div className="md:w-1/3 max-w-sm">
        <img
          src="https://tecdn.b-cdn.net/img/Photos/new-templates/bootstrap-login-form/draw2.webp"
          alt="auth image" />
      </div>

      <div className="md:w-1/3 max-w-sm">
        {msg.success !== "" && (
          <div className={"text-center mb-4 text-green-500"}>
            {msg.success}
          </div>
        )}
        {msg.error !== "" && (
          <div className={"text-center mb-4 text-red-500"}>
            {msg.error}
          </div>
        )}
        <input className="text-sm w-full px-4 py-2 border border-solid border-gray-300 rounded" value={state.username} onChange={handleUsernameChange} type="text" placeholder="Username" />
        <input className="text-sm w-full px-4 py-2 border border-solid border-gray-300 rounded mt-4" value={state.password} onChange={handlePasswordChange} type="password" placeholder="Password" />
        <div className="mt-4 flex justify-between font-semibold text-sm">
        </div>
        <div className="text-center md:text-left">
          <button onClick={() => action === "Register" ? handleRegister() : handleLogin()} disabled={state.username === "" && state.password === ""} className="mt-4 bg-blue-600 hover:bg-blue-700 px-4 py-2 text-white uppercase rounded text-xs tracking-wider" type="submit">{action}</button>
        </div>
        <div className="mt-4 font-semibold text-sm text-slate-500 text-center md:text-left">
          {action === "Login" ? "Don't have an account?" : "Already registerd?"} <a className="text-red-600 hover:underline hover:underline-offset-4 cursor-pointer" onClick={() => setAction(prev => prev === "Login" ? "Register" : "Login")}>{action === "Login" ? "Register" : "Login"}</a>
        </div>
      </div>
    </section>
  );
}

export default App;