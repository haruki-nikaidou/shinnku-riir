/* @refresh reload */
import { render } from 'solid-js/web'

import './index.css'
import {Route, Router} from '@solidjs/router'
import {HomePage} from "./pages/(home)";

const root = document.getElementById('root')

render(() => (
    <Router>
      <Route path='/' component={HomePage}/>
    </Router>
), root!)
