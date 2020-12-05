import { css } from "@emotion/css";
import React, { FC, useState } from "react";
import LoginBackground from "../components/login/LoginBackground";
import LoginForm from "../components/login/LoginForm";

import { promisified } from "tauri/api/tauri";

const Login: FC = () => {

  const onSubmit = async (
    email: string,
    password: string,
    force = false,
  ) => {
    try {
      const status = await promisified({cmd: 'login', email, password, force});
      alert('Status: ' + status);
    } catch (e) {
      alert('Error: ' + e);
    }
  };

  return (
    <LoginBackground>
      <LoginForm onSubmit={onSubmit} />
    </LoginBackground>
  );
};

export default Login;
