import styled from '@emotion/styled';
import React from 'react';
import kiwi from '../../assets/images/kiwi.svg';

const Kiwi = styled.img`
  position: fixed;
  width: 70vw;
  height: 70vh;
  right: -10%;
  bottom: -33%;
  mix-blend-mode: overlay;
  opacity: 0.5;
  user-select: none;
  -webkit-user-drag: none;
`;

export default (): JSX.Element => <Kiwi src={kiwi}/>;
