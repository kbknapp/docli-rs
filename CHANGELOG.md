<a name="v0.1.0-beta"></a>
## v0.1.0-beta (2015-06-05)


#### Bug Fixes

*   --noconfirm works properly now ([3ccffa3d](https://github.com/kbknapp/docli-rs/commit/3ccffa3d433530bef2aacddf33fea3531ad09643))

#### Improvements

*   fixes and verifies droplet commands ([d8e0fbe6](https://github.com/kbknapp/docli-rs/commit/d8e0fbe6c142244eb08ae7111ffcef77133902bc), closes [#7](https://github.com/kbknapp/docli-rs/issues/7))
*   verifies and fixes droplet commands ([3599bdb6](https://github.com/kbknapp/docli-rs/commit/3599bdb6cffb43f67a8efc4ca0a1beee9f03f125))
*   fixes and verifies image commands ([b46af340](https://github.com/kbknapp/docli-rs/commit/b46af3408b654387721a9fef686a1e1cc55bfa6d), closes [#6](https://github.com/kbknapp/docli-rs/issues/6))
*   fixes and verifies dns commands ([a2b5be1b](https://github.com/kbknapp/docli-rs/commit/a2b5be1b2d3827cffd7b0b31f7f174072ec14bd9), closes [#11](https://github.com/kbknapp/docli-rs/issues/11))
*   fixes and confirms ssh-keys commands ([2c5e25b2](https://github.com/kbknapp/docli-rs/commit/2c5e25b24e70d1f829f223f54893b10548f987a5), closes [#10](https://github.com/kbknapp/docli-rs/issues/10))
*   fixes output for droplets upgrades commands ([8076a8ab](https://github.com/kbknapp/docli-rs/commit/8076a8ab4f482297c80b02b8501b0bd192463a75), closes [#8](https://github.com/kbknapp/docli-rs/issues/8))
*   finalizes domains command ([f59edfde](https://github.com/kbknapp/docli-rs/commit/f59edfde1ecd919a651064ab9ef4cca91d2fe9be), closes [#9](https://github.com/kbknapp/docli-rs/issues/9))
*   fixes list account-actions and list domains output ([b470e125](https://github.com/kbknapp/docli-rs/commit/b470e125ecf16e72cf55cf2a430670cc4d54f25b), closes [#5](https://github.com/kbknapp/docli-rs/issues/5))
*   fixes list ssh-keys output ([e0d127be](https://github.com/kbknapp/docli-rs/commit/e0d127be5927282e5d4d18826fcffa7f658a9980))
*   fixes list images output ([01788ec1](https://github.com/kbknapp/docli-rs/commit/01788ec1338c353830151e71415f4b1e3912b185))
*   fixes list sizes command output ([29397557](https://github.com/kbknapp/docli-rs/commit/29397557c32454c5de1a40aef99fcf13ba82173d))
*   fixes output of list regions command ([e1fc8541](https://github.com/kbknapp/docli-rs/commit/e1fc8541105a5675b964d8eb535fb998f624b526))
*   improves output of account action <id> command ([0b14dfcf](https://github.com/kbknapp/docli-rs/commit/0b14dfcfefa581fb58776b42e9a04f8181734697), closes [#4](https://github.com/kbknapp/docli-rs/issues/4))
*   improves output of account actions ([c2bd47c1](https://github.com/kbknapp/docli-rs/commit/c2bd47c19266418908fc21c785857176a336c055))
*   fixes indentation of account output ([799b4d70](https://github.com/kbknapp/docli-rs/commit/799b4d70cb70e4f47639233e99c972b7b099404d))



<a name="v0.1.0-alpha2"></a>
## v0.1.0-alpha2 (2015-06-04)


#### Improvements

* **Confirmation**  adds confirmation to destructive API calls ([05be028c](https://github.com/kbknapp/docli-rs/commit/05be028c41fac75abee50e767214d97bcf33ca27), closes [#3](https://github.com/kbknapp/docli-rs/issues/3))

#### Bug Fixes

*   fixes some typos and command wording ([b083e711](https://github.com/kbknapp/docli-rs/commit/b083e711078593937f057eeca9460c748e66407b))

#### Documentation

*   updates readme ([ab1e4153](https://github.com/kbknapp/docli-rs/commit/ab1e4153153032848d66e9b043196a799051cdd2))

#### Features

* **account**
  *  implements account subcommand ([2263f0ba](https://github.com/kbknapp/docli-rs/commit/2263f0baef9990a5af85d763844b43d0a2926850))
  *  implements account subcommand ([eb5cd17a](https://github.com/kbknapp/docli-rs/commit/eb5cd17af3e123f9ca25b2fcbd328b240b53915e))
* **authentication**  authenticates with DO_AUTH_TOKEN env var or --token <token> ([23f95757](https://github.com/kbknapp/docli-rs/commit/23f9575781fba12086855315f625fe24bfe44fcf))



