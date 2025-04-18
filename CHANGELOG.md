# Changelog
All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.22.0](https://github.com/denehoffman/ganesh/compare/v0.21.1...v0.22.0) - 2025-04-11

### Added

- add `NopAbortSignal` to the main namespace
- Add AbortSignal to handle abortion of calculations
- Add AbortSignal to handle abortion of calculations
- Add AbortSignal to handle abortion of calculations

### Fixed

- add defaults to abort signals and make some `new` methods `const`

### Other

- Merge pull request #71 from estriv/main
- add abort signals to doctests

## [0.21.1](https://github.com/denehoffman/ganesh/compare/v0.21.0...v0.21.1) - 2025-04-11

### Added

- add Clone derive to TrackingSwarmObserver
- make the TrackingSwarmObserver fields pub

## [0.21.0](https://github.com/denehoffman/ganesh/compare/v0.20.1...v0.21.0) - 2025-04-11

### Added

- change the way swarm algorithms are initialized to simplify swarm construction

## [0.20.1](https://github.com/denehoffman/ganesh/compare/v0.20.0...v0.20.1) - 2025-04-10

### Added

- add Default impl for Particle

### Fixed

- update some pub visibility on Point and remove a python file committed by mistake

## [0.20.0](https://github.com/denehoffman/ganesh/compare/v0.19.0...v0.20.0) - 2025-04-10

### Added

- finalize PSO implementation and overhaul organization
- add Global step to ESS

### Fixed

- remove `kmeans` dependency and implement the algorithm by hand
- move vector generating utilities to root module

## [0.19.0](https://github.com/denehoffman/ganesh/compare/v0.18.0...v0.19.0) - 2025-04-02

### Other

- reverse a bit of the previous overhaul, keeping bounds in the initialization method and in the minimizer is a good idea, but they are not required elsewhere

## [0.18.0](https://github.com/denehoffman/ganesh/compare/v0.17.1...v0.18.0) - 2025-04-01

### Added

- reorganize, make Bounds a part of functions and individual algorithms

### Other

- remove some spaces
- move Observer traits to `observer` module

## [0.17.1](https://github.com/denehoffman/ganesh/compare/v0.17.0...v0.17.1) - 2025-04-01

### Added

- allow users to implement a tuning step at the function level

### Fixed

- correct behavior in observer doctest

## [0.17.0](https://github.com/denehoffman/ganesh/compare/v0.16.0...v0.17.0) - 2025-01-28

### Added

- allow users to skip Hessian calculation in L-BFGS-B algorithm

## [0.16.0](https://github.com/denehoffman/ganesh/compare/v0.15.2...v0.16.0) - 2025-01-04

### Other

- remove coverage tests for  feature, since it doesn't work all the time yet
- remove  from  and  since it requires  and  generics to implement , which is sometimes difficult. Also made  the default  method and removed

## [0.15.2](https://github.com/denehoffman/ganesh/compare/v0.15.1...v0.15.2) - 2024-12-20

### Other

- *(observers)* fix doctests

## [0.15.1](https://github.com/denehoffman/ganesh/compare/v0.15.0...v0.15.1) - 2024-12-20

### Fixed

- change signature of `with_observer` methods to be more flexible

## [0.15.0](https://github.com/denehoffman/ganesh/compare/v0.14.3...v0.15.0) - 2024-12-20

### Added

- Add autocorrelation `MCMCObserver`
- add `build` methods to observers to make `Arc<RwLock<Self>>`s out of them

### Fixed

- change `build` signature for standalone structs
- correct autocorrelation calculation

### Other

- add usage doctest for `AutocorrelationObserver`
- update example with IAT observer and proper burn-in, as well as an IAT plot
- remove methods to set all observers and change the signature of `with_observer` to accept an `Arc<RwLock<Observer>>`
- switch `Observer`/`MCMCObserver` callback notation to terminate if true is returned

## [0.14.3](https://github.com/denehoffman/ganesh/compare/v0.14.2...v0.14.3) - 2024-12-14

### Fixed

- make  custom constructors public

## [0.14.2](https://github.com/denehoffman/ganesh/compare/v0.14.1...v0.14.2) - 2024-12-14

### Added

- add serde to  and members

## [0.14.1](https://github.com/denehoffman/ganesh/compare/v0.14.0...v0.14.1) - 2024-12-14

### Fixed

- ensure `f32` feature works by making a few type-agnostic calls

## [0.14.0](https://github.com/denehoffman/ganesh/compare/v0.13.1...v0.14.0) - 2024-12-13

### Added

- add interface for setting Sokal window size
- add interface to update `ESS` hyperparameter settings
- finishing touches on `mcmc` module
- add integrated autocorrelation time
- add ctrl-c back to MCMC sampler
- add Multivariate normal ESS example
- add initial MCMC interface
- use `mul_add` where applicable
- add benchmarks for each algorithm
- move `Point` to be usable by other algorithms and correct the way Nelder-Mead functions are evaluated with bounds

### Other

- fix Just -> just
- remove trace plots
- update README.md and crate-level documnetation with MCMC section
- ignore .pkl files
- rename `Point::len` to `Point::dimension`
- rename i -> step
- add docstrings to sampler
- rename AIMES -> AIES
- get rid of generics

## [0.13.1](https://github.com/denehoffman/ganesh/compare/v0.13.0...v0.13.1) - 2024-11-08

### Added

- use unzip to support 1.69.0 as MSRV

### Fixed

- clippy lints

### Other

- add codspeed link to readme
- change indents

## [0.13.0](https://github.com/denehoffman/ganesh/compare/v0.12.2...v0.13.0) - 2024-11-05

### Added

- add optional parameter names field to `Status`
- add `serde` serialization and deserialization to `Status`

### Other

- switch to codspeed for benchmarks

## [0.12.2](https://github.com/denehoffman/ganesh/compare/v0.12.1...v0.12.2) - 2024-10-24

### Fixed

- change default tolerances to be a bit more realistic

## [0.12.1](https://github.com/denehoffman/ganesh/compare/v0.12.0...v0.12.1) - 2024-10-21

### Fixed

- store `Algorithm` in a `Box` in `Minimizer` and add `new_from_box` constructor
- add  trait bound to  constructor

### Other

- add test for  constructors

## [0.12.0](https://github.com/denehoffman/ganesh/compare/v0.11.3...v0.12.0) - 2024-10-21

### Added

- use `dyn-clone` to pass `Algorithm`s to `Minimizer`s by reference

### Other

- add criterion benchmark comparison for PRs
- update documentation to reflect the output of pretty-printing a `Status`

## [0.11.3](https://github.com/denehoffman/ganesh/compare/v0.11.2...v0.11.3) - 2024-10-19

### Added

- add a feature to make Ctrl-C gracefully stop the algorithm and return the fit result with a message

## [0.11.2](https://github.com/denehoffman/ganesh/compare/v0.11.1...v0.11.2) - 2024-10-17

### Added

- allow `Observer`s to modify fit `Status` and terminate minimization

## [0.11.1](https://github.com/denehoffman/ganesh/compare/v0.11.0...v0.11.1) - 2024-10-17

### Other

- relocate html docs header
- fix link in README
- Merge branch 'main' of https://github.com/denehoffman/ganesh
- correct default in docstring
- add comma

## [0.11.0](https://github.com/denehoffman/ganesh/compare/v0.10.0...v0.11.0) - 2024-09-12

### Added

- add `Display` to `Minimizer`
- add initial value and bounds to `Status` and improve `Display` output
- add (sketchy) function to check if a parameter is at its bounds

### Fixed

- add `UpperExp` trait bound to `Minimizer` `Display`
- make sure `Status` is reset on a new run of the same minimizer

### Other

- move all `Status` structs out of `Algorithm`s and into `Minimizer` and `Algorithm` method signatures

## [0.10.0](https://github.com/denehoffman/ganesh/compare/v0.9.1...v0.10.0) - 2024-09-10

### Added

- move Hessian inversion to `Status` and add `hess` field

### Fixed

- use cargo-llvm-cov (messed up git history on previous attempt)
- use correct internal/external bounded/regular calls in all algorithms
- change finite difference delta

### Other

- Merge pull request [#31](https://github.com/denehoffman/ganesh/pull/31) from denehoffman/hotfixes
- change implementation of Hessian to use gradients
- fix link

## [0.9.1](https://github.com/denehoffman/ganesh/compare/v0.9.0...v0.9.1) - 2024-09-09

### Fixed

- ensure all algorithms reset completely when their initialize method is called
- use set_cov to also calculate errors, change method to take an `Option`
- update BFGS and L-BFGS methods to be closer to the implementation for L-BFGS-B and fix errors in L-BFGS causing incorrect convergence

### Other

- update readme and main doc
- add basic convergence tests to all algorithms
- add leading signs to `Status` `Display` method
- improve pretty printing for `Status`

## [0.9.0](https://github.com/denehoffman/ganesh/compare/v0.8.5...v0.9.0) - 2024-09-09

### Added

- add errors to all algorithms
- add Hessian evaluation to `Function` trait
- kickstart BFGS with H-inverse scaling
- switch to using nalgebra data structures by default

### Fixed

- simplify L-BFGS algorithm and ensure the first few steps are computed correctly
- left and right matrices were switched by accident
- make terminator epsilon fields public for BFGS methods, set default f tolerance to epsilon rather than its cube root

## [0.8.5](https://github.com/denehoffman/ganesh/compare/v0.8.4...v0.8.5) - 2024-09-09

### Added

- reboot L-BFGS-B on invalid line searches

### Fixed

- follow strong Wolfe condition a bit more carefully
- make bounds inclusive
- ensure sufficient decrease is met before marking line search as valid
- make `g_eval` increment gradient evaluations rather than function evaluations
- use  trait to implement ordering on float-like generics

### Other

- remove unused import
- remove comment

## [0.8.4](https://github.com/denehoffman/ganesh/compare/v0.8.3...v0.8.4) - 2024-09-03

### Fixed
- use absolute value for absolute tolerance

### Other
- reverse some dot products with the wrong dimensions

## [0.8.3](https://github.com/denehoffman/ganesh/compare/v0.8.2...v0.8.3) - 2024-09-03

### Fixed
- switch sign on function termination condition

## [0.8.2](https://github.com/denehoffman/ganesh/compare/v0.8.1...v0.8.2) - 2024-09-03

### Added
- add function value terminators for BFGS algorithms
- add function value terminators for BFGS algorithms

## [0.8.1](https://github.com/denehoffman/ganesh/compare/v0.8.0...v0.8.1) - 2024-09-03

### Added
- add gradient tolerance to L-BFGS-B

### Other
- Merge branch 'main' into development
- export BFGS methods in  mod

## [0.8.0](https://github.com/denehoffman/ganesh/compare/v0.7.1...v0.8.0) - 2024-09-03

### Added
- add L-BFGS-B algorithm
- update line search to take a `max_step` optional argument and return a bool flag of validity rather than an `Option`
- add `LineSearch` trait and implementations of BFGS and L-BFGS algorithms
- update `NelderMead` to count gradient evals and use bounded interface
- add bounded evaluation shortcuts to `Function` trait and count gradient evaluations in `Status`

### Fixed
- simplify logic by removing internal `m`
- change to inequality to ensure a proper status message if the max iterations are passed

### Other
- fix brackets in readme and update main lib docs
- update readme
- remove unused collections module

## [0.7.1](https://github.com/denehoffman/ganesh/compare/v0.7.0...v0.7.1) - 2024-08-23

### Other
- fix doctests
- make minimize return `Result<(), E>` and store `Status` in the `Minimizer` struct

## [0.7.0](https://github.com/denehoffman/ganesh/compare/v0.6.0...v0.7.0) - 2024-08-23

### Added
- add useful assert warning for trying to construct a `NelderMead` `Simplex` with fewer than 2 points
- add check to make sure starting position is within bounds
- add display method, methods for getting `lower` and `upper` bounds, and `contains` method for `Bounds`
- add `Debug`s to `NelderMead`
- add preliminary implementation of BFGS algorithm
- add method to return the gradient and inverse of Hessian matrix

### Fixed
- remove tracking `main.rs`, which I use for quick demos
- adaptive Nelder-Mead now requires inputting the dimension
- remove out-of-bounds issue
- step direction should be opposite the gradient
- `p` is `-grad_f` so this was right all along
- allow expect in Hessian inverse function
- update BFGS algorithm to recent changes with ganesh
- change `learning_rate` to an `Option` in gradient descent

### Other
- adds documentation to all parts of crate, additionally makes some `Algorithm` methods return `Result`s now
- fix typo in example
- update dependencies
- update licensing
- switch license to MIT
- add Bounds section to TOC
- correct statements about `Function` trait in readme
- typo in readme
- update README.md
- major rewrite of library, adds experimental bounds to Nelder Mead
- qualify path to `abs` function
- Merge remote-tracking branch 'origin/bfgs' into development
- change slice to `DVector` in documentation
- update docs and fix links/footnotes

## [0.6.0](https://github.com/denehoffman/ganesh/compare/v0.5.0...v0.6.0) - 2024-08-17

### Added
- reduces the `Field` trait to use `num` traits rather than `nalgebra`'s `RealField`

### Fixed
- ensure all methods use the `Field` trait rather than just `Float` for better compatibility
- re-export `nalgebra::DVector`

### Other
- fix some of the documentation to reflect recent changes to the crate

## [0.5.0](https://github.com/denehoffman/ganesh/compare/v0.4.0...v0.5.0) - 2024-08-15

### Added
- change most slice types to `nalgebra::DVector`s to make algorithms more ergonomic, add `LineSearch` algorithms

## [0.4.0](https://github.com/denehoffman/ganesh/compare/v0.3.1...v0.4.0) - 2024-07-30

### Other
- undo changes to previous version, lifetimes make things more difficult to work with for end-users. Removed NelderMeadMessage.

## [0.3.1](https://github.com/denehoffman/ganesh/compare/v0.3.0...v0.3.1) - 2024-07-30

### Added
- change functions to references to avoid cloning any underlying function data

## [0.3.0](https://github.com/denehoffman/ganesh/compare/v0.2.0...v0.3.0) - 2024-07-19

### Added
- switch &Option<args> to Option<&args> and remove messages in favor of extending Minimizer trait
- add Send/Sync to Function

### Fixed
- change callback to no longer be optional, this just required typing None::<fn(&_)> everywhere which is way uglier than |_|{}
- make callback optional to avoid toilet bowl closure

### Other
- Merge branch 'development' of github.com:denehoffman/ganesh into development
- update crate docs to reflect new changes
- add wordmark
- add logo to readme
- remove num::Float trait dependence
- add logo
- Merge branch 'main' into development

## [0.2.0](https://github.com/denehoffman/ganesh/compare/v0.1.0...v0.2.0) - 2024-07-14

### Fixed
- re-implement args that were lost in merge
- move main traits to core module and modify gradient and hessian methods to work better at larger values

### Other
- update docstrings to reflect arguments
- Merge branch 'development' of https://github.com/denehoffman/ganesh into development
- add benchmark
- release

## [0.1.0](https://github.com/denehoffman/ganesh/releases/tag/v0.1.0) - 2024-07-13

### Added
- switch to typed-builder for Result-less builder patterns
- first commit, basic traits, some test functions, and some basic optimization algorithms

### Fixed
- ignore doctests that aren't meant to be tests

### Other
- add metadata to Cargo.toml
- create release.yml
- fix \geq symbol
- update documentation to match README and newer features
- Create LICENSE
- add workflows
- many interconnected changes that I don't care to write individual commits for
- remove BFGS algorithm (for now)
- add KaTeX header
- create README.md
