## Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [2.24.0](https://github.com/joaofelipegalvao/rustodo/compare/v2.23.0..v2.24.0) - 2026-03-12

### Features

- Add NoteFormat enum for markdown support - ([37bd07e](https://github.com/joaofelipegalvao/rustodo/commit/37bd07e8251b5d8bada21c36a2d0aa44b452d866))
- Add note preview command with glow - ([20750e1](https://github.com/joaofelipegalvao/rustodo/commit/20750e190481cd861158570f5bd5d620560d56e4))
- Enhance note add/edit with --editor flag - ([4038a98](https://github.com/joaofelipegalvao/rustodo/commit/4038a981b3803f6e1a5e6e867fee249feb2069fa))
- Show markdown hint in note show output - ([f2d2f83](https://github.com/joaofelipegalvao/rustodo/commit/f2d2f830eceea0d4fdbb171f8fdd499fea9ca700))

### Refactoring

- Reorganize CLI help and hide command aliases - ([53fc417](https://github.com/joaofelipegalvao/rustodo/commit/53fc417a512a0e4f64b66905d213334087a39e29))
## [2.23.0](https://github.com/joaofelipegalvao/rustodo/compare/v2.22.0..v2.23.0) - 2026-03-11

### Features

- Add urgency score for task sorting - ([e24e4b0](https://github.com/joaofelipegalvao/rustodo/commit/e24e4b0acb80371a4352d3b3a773d82664b76672))
- Add calendar, next, and holidays commands - ([554738c](https://github.com/joaofelipegalvao/rustodo/commit/554738c3b123ff3218c0a79844251b674b728e9a))
- Add calendar and next command rendering - ([175dc75](https://github.com/joaofelipegalvao/rustodo/commit/175dc75fdff18aba2e2030e221b9ced9086b31cd))
- Add holidays service from holidata.net - ([4a35527](https://github.com/joaofelipegalvao/rustodo/commit/4a355275e918cb4eb43c9ebaa5b9c1d9895da0d2))
- Add config module for app settings - ([c7320fc](https://github.com/joaofelipegalvao/rustodo/commit/c7320fc9951fa294360e6c59ec881e264e3a10c2))
- Enhance stats command with history tracking - ([7752e0d](https://github.com/joaofelipegalvao/rustodo/commit/7752e0db03d8315187e66a35bbf030440628ce2d))

### Refactoring

- Reorganize command modules into directories - ([92d75a9](https://github.com/joaofelipegalvao/rustodo/commit/92d75a98af8d834870e5d55b6f04550cc34f18eb))
- Remove old flat command modules - ([72ec09c](https://github.com/joaofelipegalvao/rustodo/commit/72ec09cf9c4f08d5cd5134ffa666fa0082694c4b))
- Remove legacy project_show module - ([7d73ae3](https://github.com/joaofelipegalvao/rustodo/commit/7d73ae3fa7ac0245a817d098ebf10a34d9afcbbe))
- Move date_parser and validation to utils module - ([ec6f32a](https://github.com/joaofelipegalvao/rustodo/commit/ec6f32a4485201df1b8d0b7a51c75a2e8306b2a4))
- Enhance TUI with new styling and components - ([8efa56e](https://github.com/joaofelipegalvao/rustodo/commit/8efa56e6b7e6d9aec9ce270e129a56e3878d18c2))
- Update render modules and services - ([db6881b](https://github.com/joaofelipegalvao/rustodo/commit/db6881befcc338713cb1ac379e012e28aaa22ec2))
## [2.22.0](https://github.com/joaofelipegalvao/rustodo/compare/v2.21.0..v2.22.0) - 2026-03-08

### Features

- CLI improvements - context, project lifecycle, tag hub, resource types - ([ad5d166](https://github.com/joaofelipegalvao/rustodo/commit/ad5d16664a889587016f2527f1fbc0bbe142dc43))
## [2.21.0](https://github.com/joaofelipegalvao/rustodo/compare/v2.20.0..v2.21.0) - 2026-03-05

### Features

- Add Projects, Notes and Resources with full CRUD - ([2995dbe](https://github.com/joaofelipegalvao/rustodo/commit/2995dbe9cdb58a1c86ffa116fb4e5629a5875f3d))

### Bug Fixes

- Adapt TUI to project_id architecture (remove task.project) - ([289ae7a](https://github.com/joaofelipegalvao/rustodo/commit/289ae7a060b74fe9b1735984678231fc760f3745))
## [2.20.0](https://github.com/joaofelipegalvao/rustodo/compare/v2.19.0..v2.20.0) - 2026-03-04

### Features

- Implement TUI MVP with full task management interface - ([36b4c05](https://github.com/joaofelipegalvao/rustodo/commit/36b4c05d1c5ee53a2daae9d1e0f9725d2d8895d3))

### Bug Fixes

- Resolve clippy warnings in events.rs, ui.rs and done.rs - ([7d8ff79](https://github.com/joaofelipegalvao/rustodo/commit/7d8ff79d48fc42e7fbe76f7280010ee6a56e2a8e))
- Resolve clippy warnings in events.rs, ui.rs and done.rs - ([4f0557d](https://github.com/joaofelipegalvao/rustodo/commit/4f0557dac2f9e26edd05901287bd2f36650b32c6))
## [2.19.0](https://github.com/joaofelipegalvao/rustodo/compare/v2.18.0..v2.19.0) - 2026-03-03

### Features

- Implement purge command for tombstone cleanup - ([ef257b0](https://github.com/joaofelipegalvao/rustodo/commit/ef257b05b978b86303223f8cb0e2cf06b6dbc790))
## [2.18.0](https://github.com/joaofelipegalvao/rustodo/compare/v2.17.5..v2.18.0) - 2026-03-03

### Features

- Implement soft delete for sync-safe task removal - ([d5d6567](https://github.com/joaofelipegalvao/rustodo/commit/d5d6567bc240bfa25fd5a310c88389391e54e1c9))
## [2.17.5](https://github.com/joaofelipegalvao/rustodo/compare/v2.17.4..v2.17.5) - 2026-03-03

### Bug Fixes

- Use merge instead of rebase to prevent task loss on divergent histories - ([7db6339](https://github.com/joaofelipegalvao/rustodo/commit/7db63395f47be97f541f5d0dc431f8673514ca0b))
## [2.17.4](https://github.com/joaofelipegalvao/rustodo/compare/v2.17.3..v2.17.4) - 2026-03-03

### Bug Fixes

- *(sync)* Use --force-with-lease after merge on rejected push - ([314d7f2](https://github.com/joaofelipegalvao/rustodo/commit/314d7f294fe19b762658bfda36bfe94fb9d8f185))
## [2.17.3](https://github.com/joaofelipegalvao/rustodo/compare/v2.17.2..v2.17.3) - 2026-03-03

### Bug Fixes

- Pass FETCH_HEAD JSON to Conflict branch to prevent duplicates - ([622a9dd](https://github.com/joaofelipegalvao/rustodo/commit/622a9ddef2bf09f4240a0a866594fce36ba2a5b1))
## [2.17.2](https://github.com/joaofelipegalvao/rustodo/compare/v2.17.1..v2.17.2) - 2026-03-03

### Bug Fixes

- Fetch remote JSON before merge to prevent duplicates - ([ef97470](https://github.com/joaofelipegalvao/rustodo/commit/ef974701c231501f4dc074d39cc0db51102b48bb))
## [2.17.1](https://github.com/joaofelipegalvao/rustodo/compare/v2.17.0..v2.17.1) - 2026-03-02

### Bug Fixes

- Correct pull conflict resolution using semantic merge - ([df6f8cc](https://github.com/joaofelipegalvao/rustodo/commit/df6f8ccbdec9d4ed1221c8ef7e49ac38b4ec7f3d))
## [2.17.0](https://github.com/joaofelipegalvao/rustodo/compare/v2.16.0..v2.17.0) - 2026-03-01

### Features

- Implement semantic merge for todo sync pull - ([13d9f54](https://github.com/joaofelipegalvao/rustodo/commit/13d9f547161d4c62667605573767423664d00676))
## [2.16.0](https://github.com/joaofelipegalvao/rustodo/compare/v2.15.0..v2.16.0) - 2026-03-01

### Features

- *(sync)* Improve commit message and status output - ([ec5eeba](https://github.com/joaofelipegalvao/rustodo/commit/ec5eeba84e5d244870db0382782ba0e57ec9e410))
## [2.15.0](https://github.com/joaofelipegalvao/rustodo/compare/v2.14.0..v2.15.0) - 2026-03-01

### Features

- Implement git sync via std::process::Command - ([0f736df](https://github.com/joaofelipegalvao/rustodo/commit/0f736df7828ba3a26415dd280c01f0f8fde85c5f))
- Implement git sync via std::process::Command #5 - ([c01d2e5](https://github.com/joaofelipegalvao/rustodo/commit/c01d2e51285ecfbe62434eb56f7d3e17cf82abb1))
## [2.14.0](https://github.com/joaofelipegalvao/rustodo/compare/v2.13.0..v2.14.0) - 2026-02-27

### Features

- Add GitStorage backend with automatic Git commits - ([4f4a06f](https://github.com/joaofelipegalvao/rustodo/commit/4f4a06f15dbeb6d04e259f74ebfa1287c24ce131))

### Bug Fixes

- Use vendored-openssl for git2 to fix cross-compilation - ([ea982c9](https://github.com/joaofelipegalvao/rustodo/commit/ea982c9558bf2c42bad10f736b547cad198e9ee2))
## [2.13.0](https://github.com/joaofelipegalvao/rustodo/compare/v2.12.0..v2.13.0) - 2026-02-26

### Features

- Add updated_at field to Task for sync conflict resolution - ([d96c987](https://github.com/joaofelipegalvao/rustodo/commit/d96c987f6fb6d726cde349851a7184fd9980b2e8))
## [2.12.0](https://github.com/joaofelipegalvao/rustodo/compare/v2.11.0..v2.12.0) - 2026-02-26

### Features

- Migrate depends_on and parent_id from usize to UUID - ([6df77c5](https://github.com/joaofelipegalvao/rustodo/commit/6df77c5be74a6a40db80e4fa34acfb6106b6420a))
## [2.11.0](https://github.com/joaofelipegalvao/rustodo/compare/v2.9.0..v2.11.0) - 2026-02-26

### Features

- Add stable UUIDs for future sync support - ([2d1c908](https://github.com/joaofelipegalvao/rustodo/commit/2d1c908e303083e165e0e296b954e7cd255e78ab))
## [2.9.0](https://github.com/joaofelipegalvao/rustodo/compare/v2.8.11..v2.9.0) - 2026-02-26

### Features

- Add file locking and atomic writes for safe concurrent access - ([08bc4b9](https://github.com/joaofelipegalvao/rustodo/commit/08bc4b96f54cea49da6b03466bc75a3260872faa))
## [2.8.11](https://github.com/joaofelipegalvao/rustodo/compare/v2.8.10..v2.8.11) - 2026-02-25

### Bug Fixes

- Implement atomic file writes to prevent corruption on crash - ([2e60812](https://github.com/joaofelipegalvao/rustodo/commit/2e60812f30a254b4b057c5a1b18239c290493cb8))
## [2.8.10](https://github.com/joaofelipegalvao/rustodo/compare/v2.8.9..v2.8.10) - 2026-02-25

### Refactoring

- Extract EditArgs struct and simplify edit command signature - ([9474183](https://github.com/joaofelipegalvao/rustodo/commit/94741835453da8fdc6dc548c3b9dd80720d29ae5))
## [2.8.9](https://github.com/joaofelipegalvao/rustodo/compare/v2.8.8..v2.8.9) - 2026-02-25

### Refactoring

- Simplify command handlers by accepting Args structs directly - ([ab2c0f6](https://github.com/joaofelipegalvao/rustodo/commit/ab2c0f6756605265892fad3acf7eef51a803970b))
## [2.8.8](https://github.com/joaofelipegalvao/rustodo/compare/v2.8.7..v2.8.8) - 2026-02-24

### Bug Fixes

- Update logo path in README - ([d8eb225](https://github.com/joaofelipegalvao/rustodo/commit/d8eb225b34cb105328c749e28c6adcd2d7207a95))

### Documentation

- Add Codecov and docs.rs badges - ([5a1bf6d](https://github.com/joaofelipegalvao/rustodo/commit/5a1bf6da9d4a86eaf4ec5298c311bdf3cd6c87af))
## [2.8.7](https://github.com/joaofelipegalvao/rustodo/compare/v2.8.6..v2.8.7) - 2026-02-24

### Bug Fixes

- Use automatic version from Cargo.toml in CLI - ([5bffc65](https://github.com/joaofelipegalvao/rustodo/commit/5bffc6502b5ba5352ff62211a746072a68436213))

### Documentation

- Update logo to rustodo and adjust sizing - ([b4c3896](https://github.com/joaofelipegalvao/rustodo/commit/b4c38963636ea3b1da397394bc479c3e0830f7c5))
- Improve module documentation and fix doc test examples - ([0d07c15](https://github.com/joaofelipegalvao/rustodo/commit/0d07c1592c406605bd3e68ae6c896a346883ea57))
## [2.8.6](https://github.com/joaofelipegalvao/rustodo/compare/v2.8.5..v2.8.6) - 2026-02-24

### Bug Fixes

- Correct ownership in display_lists doctest - ([6a6954e](https://github.com/joaofelipegalvao/rustodo/commit/6a6954e06fe1038aea430fb410c733f9ab32a09b))

### Documentation

- Add rustdoc comments to all public modules and types - ([1931034](https://github.com/joaofelipegalvao/rustodo/commit/1931034f3489cb1a7be905501984c410eb2b914e))
## [2.8.5](https://github.com/joaofelipegalvao/rustodo/compare/v2.8.3..v2.8.5) - 2026-02-23

### Bug Fixes

- Update CLI name to rustodo and bump version to 2.8.4 - ([e0b52ff](https://github.com/joaofelipegalvao/rustodo/commit/e0b52ffc5c977e0a42a4d9bf7a94495c687eb6b0))

### Refactoring

- Use rustodo lib crate in main.rs instead of redeclaring modules - ([526e21c](https://github.com/joaofelipegalvao/rustodo/commit/526e21c22377edf51c1230eaae66fcafffe6f136))
## [2.8.3](https://github.com/joaofelipegalvao/rustodo/compare/v2.8.2..v2.8.3) - 2026-02-23

### Bug Fixes

- Update project name from todo-cli to rustodo - ([7c46f77](https://github.com/joaofelipegalvao/rustodo/commit/7c46f77cff9cdef73762ab091bb918bc122819bb))
## [2.8.2](https://github.com/joaofelipegalvao/rustodo/compare/v2.8.1..v2.8.2) - 2026-02-23

### Bug Fixes

- Remove wildcard dependency for chrono - ([4211d19](https://github.com/joaofelipegalvao/rustodo/commit/4211d19d40b17e248202fc3fd910ba51d641c2f6))

### Documentation

- Fix crate name in doctest examples - ([4e54e41](https://github.com/joaofelipegalvao/rustodo/commit/4e54e4170267756751797760bbb091bc3be7d391))
- Update repo name from todo-cli to rustodo - ([bac5888](https://github.com/joaofelipegalvao/rustodo/commit/bac5888863ca1929e258459ef8cad4f18dd43682))
## [2.8.1](https://github.com/joaofelipegalvao/rustodo/compare/v2.8.0..v2.8.1) - 2026-02-23

### Bug Fixes

- Update test imports from todo_cli to rustodo - ([3711a3a](https://github.com/joaofelipegalvao/rustodo/commit/3711a3adcf0ed6a761df1b0d75d2284baf1866f4))

### Documentation

- Add logo, demo gif and contributing guide - ([4b493ec](https://github.com/joaofelipegalvao/rustodo/commit/4b493ecf458596e2b924b9f0b6bbd12b46420f39))
- Att GUIDE.md - ([515126e](https://github.com/joaofelipegalvao/rustodo/commit/515126e0bdec0728e6f80d627a6658058aabbfa2))
## [2.8.0](https://github.com/joaofelipegalvao/rustodo/compare/v2.7.0..v2.8.0) - 2026-02-21

### Features

- Add automatic tag normalization with fuzzy matching - ([4bcbfb2](https://github.com/joaofelipegalvao/rustodo/commit/4bcbfb2b2f5c72f90865be21d8401f60c96f7ecf))
## [2.7.0](https://github.com/joaofelipegalvao/rustodo/compare/v2.6.1..v2.7.0) - 2026-02-21

### Features

- Add stats command for productivity statistics - ([9d235a9](https://github.com/joaofelipegalvao/rustodo/commit/9d235a93d7f76bcdca3a5ff34c1937d0e6963199))
## [2.6.1](https://github.com/joaofelipegalvao/rustodo/compare/v2.6.0..v2.6.1) - 2026-02-21

### Refactoring

- Simplify error handling and if-let patterns - ([ff4ad5d](https://github.com/joaofelipegalvao/rustodo/commit/ff4ad5d84dd03b833a96a32a67185df1f3f3cad6))
## [2.6.0](https://github.com/joaofelipegalvao/rustodo/compare/v2.5.0..v2.6.0) - 2026-02-20

### Features

- Add task dependencies system with cycle detection - ([7b1f422](https://github.com/joaofelipegalvao/rustodo/commit/7b1f4223155b37bf8b0a56b1fb0bdb768db3c651))
## [2.5.0](https://github.com/joaofelipegalvao/rustodo/compare/v2.4.1..v2.5.0) - 2026-02-20

### Features

- Add project system for organizing tasks - ([7308a0b](https://github.com/joaofelipegalvao/rustodo/commit/7308a0b5592d3e791fd2dde2efc34ebfd4fb8634))
## [2.4.1](https://github.com/joaofelipegalvao/rustodo/compare/v2.4.0..v2.4.1) - 2026-02-20

### Performance

- Optimize date parsing with LazyLock and add past date validation - ([fb6c20a](https://github.com/joaofelipegalvao/rustodo/commit/fb6c20afc879fc2faba64096766152e5af3484c3))
## [2.4.0](https://github.com/joaofelipegalvao/rustodo/compare/v2.3.4..v2.4.0) - 2026-02-20

### Features

- Add natural language date parsing (NLP dates) - ([ab9fa6e](https://github.com/joaofelipegalvao/rustodo/commit/ab9fa6ef7dac76caed330ad5f9322a475b94660c))

### Bug Fixes

- *(ci)* Remove GitHub API dependency from git-cliff config - ([f64a7f8](https://github.com/joaofelipegalvao/rustodo/commit/f64a7f8d252019aebc670bcbb28abeda47e6234b))
- *(ci)* Hardcode repo URL in cliff.toml template to fix Tera scope issue - ([df40c3e](https://github.com/joaofelipegalvao/rustodo/commit/df40c3e88a7aecbd70b42f4a07daaa93be69c9a2))
- Collapse nested if-let and add GitHub token to git-cliff steps - ([ce24bfd](https://github.com/joaofelipegalvao/rustodo/commit/ce24bfdb0e9bdf674727d7a850afa5aafef7798c))
## [2.3.4](https://github.com/joaofelipegalvao/rustodo/compare/v2.3.3..v2.3.4) - 2026-02-19

### Bug Fixes

- *(ci)* Generate CHANGELOG after tag creation so version is resolved correctly - ([50101d4](https://github.com/joaofelipegalvao/rustodo/commit/50101d490fc2df9eece24877a5b5725cd4bc8698))
## [2.3.3](https://github.com/joaofelipegalvao/rustodo/compare/v2.3.2..v2.3.3) - 2026-02-19

### Bug Fixes

- *(ci)* Handle existing tags in release workflow - ([bc2b287](https://github.com/joaofelipegalvao/rustodo/commit/bc2b28780042b20ec95ca40e5c1c3db750766203))
- *(ci)* Add comparison links to CHANGELOG footer - ([dda87f6](https://github.com/joaofelipegalvao/rustodo/commit/dda87f696496bbdd870449ea395fb5d03a1343b4))
- *(ci)* Fix CHANGELOG footer template for git-cliff - ([8a4a14c](https://github.com/joaofelipegalvao/rustodo/commit/8a4a14c28187fe091ee9b65dfc68aab2e19e34c8))
- *(ci)* Sync Cargo.toml version with last git tag before bump - ([743b94e](https://github.com/joaofelipegalvao/rustodo/commit/743b94e5b787d63777a4f83c51928c371ebf7cd0))
- Test release pipeline - ([61b2197](https://github.com/joaofelipegalvao/rustodo/commit/61b21975ca7c036fe062e785774fde47103ba83a))
## [2.3.2](https://github.com/joaofelipegalvao/rustodo/compare/v2.3.1..v2.3.2) - 2026-02-19

### Refactoring

- Remove unused storage functions and fix dead code warnings - ([cf16d2b](https://github.com/joaofelipegalvao/rustodo/commit/cf16d2ba4f60cee8f86437e6ba32cb523a269103))
## [2.3.1](https://github.com/joaofelipegalvao/rustodo/compare/v2.3.0..v2.3.1) - 2026-02-19

### Bug Fixes

- Remove unused PathBuf import in storage module - ([0a4f37f](https://github.com/joaofelipegalvao/rustodo/commit/0a4f37f8b0c26ab15002dfbcc29da3882b00921c))
## [2.3.0](https://github.com/joaofelipegalvao/rustodo/compare/v2.2.2..v2.3.0) - 2026-02-19

### Features

- *(search)* Add status filter to search command - ([8f3deb4](https://github.com/joaofelipegalvao/rustodo/commit/8f3deb4a69547eaea3d390d3ec956013f983921a))
## [2.2.2](https://github.com/joaofelipegalvao/rustodo/compare/v2.2.1..v2.2.2) - 2026-02-19

### Bug Fixes

- Update markdownlint config and remove mkdocs link from README - ([a2faad0](https://github.com/joaofelipegalvao/rustodo/commit/a2faad0e559366831eb24f7e8cef66684e161383))
## [2.2.1](https://github.com/joaofelipegalvao/rustodo/compare/v2.2.0..v2.2.1) - 2026-02-19

### Bug Fixes

- Remove unused import and collapse nested if, fix README link - ([fd7646f](https://github.com/joaofelipegalvao/rustodo/commit/fd7646f342eda0d3fa367eab93cec37b34540cb5))
## [2.2.0](https://github.com/joaofelipegalvao/rustodo/compare/v2.1.0..v2.2.0) - 2026-02-18

### Features

- *(edit)* Add --add-tag and --remove-tag with comma support - ([ec151a4](https://github.com/joaofelipegalvao/rustodo/commit/ec151a4c867af191f3b0362f8909694d92713d5c))

### Refactoring

- *(commands)* Update to use new validation and storage modules - ([120c93f](https://github.com/joaofelipegalvao/rustodo/commit/120c93f1f9958a74f2763115e21f66dbac876df6))
- *(error)* Add validation error variants - ([2f5032a](https://github.com/joaofelipegalvao/rustodo/commit/2f5032a9d9bf55988ef57e9b245f984d4c9d76c8))
- *(lib)* Create library crate structure - ([3a8695c](https://github.com/joaofelipegalvao/rustodo/commit/3a8695cec7b2d88171c42125a5bc0ec896d993a7))
- *(main)* Update binary to use library - ([eb14fcb](https://github.com/joaofelipegalvao/rustodo/commit/eb14fcbdcb8dfe215e842ce11b95ca806a7a5930))
- *(models)* Add Display impl for Recurrence and fix doc tests - ([c5598fc](https://github.com/joaofelipegalvao/rustodo/commit/c5598fc06e0344ae17e8ad0c810034a61051a5e1))
- *(storage)* Extract storage trait with json/memory implementations - ([7bda7e3](https://github.com/joaofelipegalvao/rustodo/commit/7bda7e3c54e13d47f82b16e515ade670290f75a4))
- *(validation)* Expand validation module with comprehensive checks - ([a5f3e6d](https://github.com/joaofelipegalvao/rustodo/commit/a5f3e6dfadd45e42f32ca516344c9acf39e44db0))
## [2.1.0](https://github.com/joaofelipegalvao/rustodo/compare/v2.0.0..v2.1.0) - 2026-02-12

### Features

- *(cli)* Add recur/norecur commands with examples - ([c342b83](https://github.com/joaofelipegalvao/rustodo/commit/c342b83bc22705bddbda9be2530e31d9b2ecf7b6))
- *(commands)* Add recur and norecur commands - ([b3feb30](https://github.com/joaofelipegalvao/rustodo/commit/b3feb3093112c695a65af5c9ab08e6eb950b197d))
- *(display)* Add recurrence column to table (D/W/M indicators) - ([7247390](https://github.com/joaofelipegalvao/rustodo/commit/7247390590a51543a1215920b43a938e68d80f3a))
- *(done)* Auto-create next recurrence when completing recurring task - ([1e3ea78](https://github.com/joaofelipegalvao/rustodo/commit/1e3ea78f7b29fa00e4dcec11c91d9fac7ce0b09e))
- *(list)* Add recurrence filters (daily/weekly/monthly/recurring/non-recurring) - ([7fbf722](https://github.com/joaofelipegalvao/rustodo/commit/7fbf722ec8e8b25faf4f62012b9b57d6aa054892))
- *(models)* Add Recurrence enum and task recurrence support - ([b7d173a](https://github.com/joaofelipegalvao/rustodo/commit/b7d173a57b6fc1d41bb095b1cdd74aeba0b63094))

### Refactoring

- *(commands)* Improve feedback messages and validation - ([e232ba6](https://github.com/joaofelipegalvao/rustodo/commit/e232ba6da976c6a2c700ab68d1dc144ad2a9ed7a))

### Documentation

- *(advanced)* Add v2.1.0 recurring tasks guide - ([f5ce3d1](https://github.com/joaofelipegalvao/rustodo/commit/f5ce3d118f2a9b1ab765ae4987080d7b50b374c0))
- *(guide)* Document recurring tasks - ([208c38e](https://github.com/joaofelipegalvao/rustodo/commit/208c38ed00226c7b7fcc2a3552a761576a25264a))
- *(readme)* Update with recurring tasks features - ([c70fe76](https://github.com/joaofelipegalvao/rustodo/commit/c70fe76dadee2b46e380e371f45bf42daae82d27))
- Update mkdocs navigation - ([d574d78](https://github.com/joaofelipegalvao/rustodo/commit/d574d787c07ea5f1770c513054b963fbe60963ba))
## [2.0.0](https://github.com/joaofelipegalvao/rustodo/compare/v1.9.0..v2.0.0) - 2026-02-10

### Refactoring

- [**breaking**] Modularize architecture and split monolithic main.rs - ([b5b2d7c](https://github.com/joaofelipegalvao/rustodo/commit/b5b2d7c51be2a8f7e4e861964018e1fb299c6fe2))

### Documentation

- *(mkdocs)* Document v2.0 modular architecture refactor - ([7e731d2](https://github.com/joaofelipegalvao/rustodo/commit/7e731d2196a84084b62d90f6aa5e41daf62aa84b))
- *(readme)* V2.0.0 modular architecture refactor - ([ed37b35](https://github.com/joaofelipegalvao/rustodo/commit/ed37b351436b5bfca97fa5d3349e51c92eb30b8c))
## [1.9.0](https://github.com/joaofelipegalvao/rustodo/compare/v1.8.0..v1.9.0) - 2026-02-10

### Features

- *(edit)* Add edit command and interactive confirmation prompts - ([d201bda](https://github.com/joaofelipegalvao/rustodo/commit/d201bdadc47ef8fce3564c829a5fe1dd4beb2c1e))

### Refactoring

- *(ui)* Centralize table layout with TableLayout - ([32a88b6](https://github.com/joaofelipegalvao/rustodo/commit/32a88b61717f0cc75a7c7a838550b080d49065ca))

### Documentation

- *(guide)* Document edit command and confirmation prompts - ([027bf6c](https://github.com/joaofelipegalvao/rustodo/commit/027bf6c89732b2050e6b74adf95d12ec127eb44a))
- *(mkdocs)* Document TableLayout architecture and layout decisions - ([131528f](https://github.com/joaofelipegalvao/rustodo/commit/131528f0ef9cf46cd713e5d3dc6e0c5ba7153f91))
- *(readme)* Highlight TableLayout-based display architecture - ([00d6e75](https://github.com/joaofelipegalvao/rustodo/commit/00d6e75ed1c44ec2c3db4ced124cdfadcd208e60))
## [1.8.0](https://github.com/joaofelipegalvao/rustodo/compare/v1.7.0..v1.8.0) - 2026-02-09

### Features

- *(info)* Add command to display data file location - ([7565418](https://github.com/joaofelipegalvao/rustodo/commit/7565418e0eeb2430cea4910296fb9f185eaee1f5))
- *(info)* Add command to display data file location and doc comments - ([71182e2](https://github.com/joaofelipegalvao/rustodo/commit/71182e2bb1c0626d71f614599d8fda4d5b58a604))
- [**breaking**] Migrate task storage to OS configuration directory - ([d1a0dd0](https://github.com/joaofelipegalvao/rustodo/commit/d1a0dd0b871ca1042cba4201a88e3f6cb0e407a6))

### Documentation

- Document v1.8.0 global data directory feature - ([a155523](https://github.com/joaofelipegalvao/rustodo/commit/a155523ba27e0b53d3e3db3965342482cec80464))
- Fix examples and explanations for global data directory - ([b0affd7](https://github.com/joaofelipegalvao/rustodo/commit/b0affd7780e8e2878fd87e5f3417cdb7c7a58a46))
- Document global data directory and info command - ([70abd90](https://github.com/joaofelipegalvao/rustodo/commit/70abd909b8e5568a0d32088c8806d9deaacf9324))
## [1.7.0](https://github.com/joaofelipegalvao/rustodo/compare/v1.6.0..v1.7.0) - 2026-02-07

### Features

- [**breaking**] Professional error handling with anyhow and thiserror - ([da4b40d](https://github.com/joaofelipegalvao/rustodo/commit/da4b40d1217d04cc6f7d99d4543c460233dbf496))

### Documentation

- Migrate project documentation to MkDocs - ([d65332b](https://github.com/joaofelipegalvao/rustodo/commit/d65332b08d31270f886957c10158b43f931f218a))
## [1.6.0](https://github.com/joaofelipegalvao/rustodo/compare/v1.5.0..v1.6.0) - 2026-02-04

### Features

- [**breaking**] V1.6.0 - professional CLI with clap - ([9ac0ca8](https://github.com/joaofelipegalvao/rustodo/commit/9ac0ca8d50fffc32aef8f54d67089e1013c66509))
## [1.5.0](https://github.com/joaofelipegalvao/rustodo/compare/v1.4.0..v1.5.0) - 2026-02-04

### Features

- [**breaking**] V1.5.0 - due dates, sorting, and tabular task display - ([5c6da3a](https://github.com/joaofelipegalvao/rustodo/commit/5c6da3a797eca3040aa1e90b771b0f771e14ae64))
## [1.4.0](https://github.com/joaofelipegalvao/rustodo/compare/v1.3.0..v1.4.0) - 2026-02-02

### Features

- V1.4.0 - tags system and correct task numbering - ([3f0faa5](https://github.com/joaofelipegalvao/rustodo/commit/3f0faa533e2298c658a670bd67a3f6843677f01d))
## [1.3.0](https://github.com/joaofelipegalvao/rustodo/compare/v1.2.0..v1.3.0) - 2026-01-30

### Features

- V1.3.0 - JSON serialization with serde - ([d8f5ea9](https://github.com/joaofelipegalvao/rustodo/commit/d8f5ea981707e056b1ef5a1c6c1b1d116587a67f))
## [1.2.0](https://github.com/joaofelipegalvao/rustodo/compare/v1.1.0..v1.2.0) - 2026-01-30

### Features

- V1.2.0 - type-safe task architecture with structs and enums - ([f4e9385](https://github.com/joaofelipegalvao/rustodo/commit/f4e9385cdfb7451b3c6725bb95944a412d123f98))
## [1.1.0](https://github.com/joaofelipegalvao/rustodo/compare/v1.0.0..v1.1.0) - 2026-01-28

### Features

- [**breaking**] V1.0.1 - translate entire codebase to English - ([e231654](https://github.com/joaofelipegalvao/rustodo/commit/e2316540ad83118558fc4ae6bc70815dd848472d))
- V1.1.0 - add --medium priority filter - ([c83dd94](https://github.com/joaofelipegalvao/rustodo/commit/c83dd949c00f54df0af6e19698d51c0aa7098db2))
## [1.0.0](https://github.com/joaofelipegalvao/rustodo/compare/v0.9.0..v1.0.0) - 2026-01-27

### Features

- V1.0.0 - search command + architectural refactoring - ([8af90a4](https://github.com/joaofelipegalvao/rustodo/commit/8af90a404cb9503236137e0e124d3205cf3d37bd))
## [0.9.0](https://github.com/joaofelipegalvao/rustodo/compare/v0.8.0..v0.9.0) - 2026-01-27

### Features

- V0.9.0 - priority sorting with --sort flag - ([0ae4962](https://github.com/joaofelipegalvao/rustodo/commit/0ae49622620d94dffd76e8d167abcae3d371444f))
## [0.8.0](https://github.com/joaofelipegalvao/rustodo/compare/v0.7.0..v0.8.0) - 2026-01-27

### Features

- V0.8.0 - priority system with advanced filters - ([a72e487](https://github.com/joaofelipegalvao/rustodo/commit/a72e487b958ee99d0b5dce9acc2249a7d6901d6b))
## [0.7.0](https://github.com/joaofelipegalvao/rustodo/compare/v0.6.0..v0.7.0) - 2026-01-26

### Features

- V0.7.0 - advanced filters with flags - ([696511f](https://github.com/joaofelipegalvao/rustodo/commit/696511f2093e716490228f3295250f88bcead284))

### Documentation

- Update README for v0.6.0 colored interface - ([75295a9](https://github.com/joaofelipegalvao/rustodo/commit/75295a92d360c03aa95836c412a5ed58a41eb1ef))
## [0.6.0](https://github.com/joaofelipegalvao/rustodo/compare/v0.5.0..v0.6.0) - 2026-01-25

### Features

- V0.6.0 - colored interface with progress tracking - ([475e0c0](https://github.com/joaofelipegalvao/rustodo/commit/475e0c0eccfcdb7feaf01fde12b53c454367b091))

### Documentation

- Complete README restructure with comprehensive documentation - ([7f5135c](https://github.com/joaofelipegalvao/rustodo/commit/7f5135c88e5c8f9ece2a80d00f4ac84f06bcab67))
## [0.5.0](https://github.com/joaofelipegalvao/rustodo/compare/v0.4.2..v0.5.0) - 2026-01-24

### Features

- V5 - clear command to remove all tasks - ([d443e1b](https://github.com/joaofelipegalvao/rustodo/commit/d443e1b026358bd287ea08f94fe64466046f86b9))
## [0.4.2](https://github.com/joaofelipegalvao/rustodo/compare/v0.4.1..v0.4.2) - 2026-01-24

### Features

- V4.2 - add state validation for task operations - ([29f1b0a](https://github.com/joaofelipegalvao/rustodo/commit/29f1b0a2ec6c84f176f6decc61086e553470a705))
## [0.4.1](https://github.com/joaofelipegalvao/rustodo/compare/v0.4.0..v0.4.1) - 2026-01-24

### Bug Fixes

- Display bug in list command showing empty lines - ([9a84730](https://github.com/joaofelipegalvao/rustodo/commit/9a84730445b6a8cda7e733307b07fb0af7da27c5))
## [0.4.0](https://github.com/joaofelipegalvao/rustodo/compare/v0.3.0..v0.4.0) - 2026-01-23

### Features

- V4 - undone command for task completion toggle - ([630f2e1](https://github.com/joaofelipegalvao/rustodo/commit/630f2e1ec8ba118d36e7e89c2e2e3e1157e9416a))
## [0.3.0](https://github.com/joaofelipegalvao/rustodo/compare/v0.2.0..v0.3.0) - 2026-01-23

### Features

- V3 - remove command for task deletion - ([17bbf1a](https://github.com/joaofelipegalvao/rustodo/commit/17bbf1ac3d4f5568a14f102a534560fda9288e95))
## [0.2.0](https://github.com/joaofelipegalvao/rustodo/compare/v0.1.0..v0.2.0) - 2026-01-23

### Features

- Add v2 - done command with task completion - ([a562656](https://github.com/joaofelipegalvao/rustodo/commit/a5626567103c1faa69c96caea1cab27ad6f89b14))

### Bug Fixes

- Add input validation for done command - ([9c09fea](https://github.com/joaofelipegalvao/rustodo/commit/9c09fead1060c16c70f3ff746000a1304ecab9ed))

### Documentation

- V1 - basic todo CLI with add/list commands - ([f2354c9](https://github.com/joaofelipegalvao/rustodo/commit/f2354c9d7cfda27dd4068954fd72ab7f44a11c3b))
- V2 - done command with task completion - ([26d6abe](https://github.com/joaofelipegalvao/rustodo/commit/26d6abe4f1f9db639e1794b58e9b9d509bb6d754))
## [0.1.0] - 2026-01-23

### Features

- V1 - basic todo CLI with add/list commands - ([9580ae2](https://github.com/joaofelipegalvao/rustodo/commit/9580ae297837c9a6c5d4b18868d2f3abac1b1b9e))
[2.24.0]: https://github.com/joaofelipegalvao/rustodo/compare/v2.23.0...v2.24.0
[2.23.0]: https://github.com/joaofelipegalvao/rustodo/compare/v2.22.0...v2.23.0
[2.22.0]: https://github.com/joaofelipegalvao/rustodo/compare/v2.21.0...v2.22.0
[2.21.0]: https://github.com/joaofelipegalvao/rustodo/compare/v2.20.0...v2.21.0
[2.20.0]: https://github.com/joaofelipegalvao/rustodo/compare/v2.19.0...v2.20.0
[2.19.0]: https://github.com/joaofelipegalvao/rustodo/compare/v2.18.0...v2.19.0
[2.18.0]: https://github.com/joaofelipegalvao/rustodo/compare/v2.17.5...v2.18.0
[2.17.5]: https://github.com/joaofelipegalvao/rustodo/compare/v2.17.4...v2.17.5
[2.17.4]: https://github.com/joaofelipegalvao/rustodo/compare/v2.17.3...v2.17.4
[2.17.3]: https://github.com/joaofelipegalvao/rustodo/compare/v2.17.2...v2.17.3
[2.17.2]: https://github.com/joaofelipegalvao/rustodo/compare/v2.17.1...v2.17.2
[2.17.1]: https://github.com/joaofelipegalvao/rustodo/compare/v2.17.0...v2.17.1
[2.17.0]: https://github.com/joaofelipegalvao/rustodo/compare/v2.16.0...v2.17.0
[2.16.0]: https://github.com/joaofelipegalvao/rustodo/compare/v2.15.0...v2.16.0
[2.15.0]: https://github.com/joaofelipegalvao/rustodo/compare/v2.14.0...v2.15.0
[2.14.0]: https://github.com/joaofelipegalvao/rustodo/compare/v2.13.0...v2.14.0
[2.13.0]: https://github.com/joaofelipegalvao/rustodo/compare/v2.12.0...v2.13.0
[2.12.0]: https://github.com/joaofelipegalvao/rustodo/compare/v2.11.0...v2.12.0
[2.11.0]: https://github.com/joaofelipegalvao/rustodo/compare/v2.9.0...v2.11.0
[2.9.0]: https://github.com/joaofelipegalvao/rustodo/compare/v2.8.11...v2.9.0
[2.8.11]: https://github.com/joaofelipegalvao/rustodo/compare/v2.8.10...v2.8.11
[2.8.10]: https://github.com/joaofelipegalvao/rustodo/compare/v2.8.9...v2.8.10
[2.8.9]: https://github.com/joaofelipegalvao/rustodo/compare/v2.8.8...v2.8.9
[2.8.8]: https://github.com/joaofelipegalvao/rustodo/compare/v2.8.7...v2.8.8
[2.8.7]: https://github.com/joaofelipegalvao/rustodo/compare/v2.8.6...v2.8.7
[2.8.6]: https://github.com/joaofelipegalvao/rustodo/compare/v2.8.5...v2.8.6
[2.8.5]: https://github.com/joaofelipegalvao/rustodo/compare/v2.8.3...v2.8.5
[2.8.3]: https://github.com/joaofelipegalvao/rustodo/compare/v2.8.2...v2.8.3
[2.8.2]: https://github.com/joaofelipegalvao/rustodo/compare/v2.8.1...v2.8.2
[2.8.1]: https://github.com/joaofelipegalvao/rustodo/compare/v2.8.0...v2.8.1
[2.8.0]: https://github.com/joaofelipegalvao/rustodo/compare/v2.7.0...v2.8.0
[2.7.0]: https://github.com/joaofelipegalvao/rustodo/compare/v2.6.1...v2.7.0
[2.6.1]: https://github.com/joaofelipegalvao/rustodo/compare/v2.6.0...v2.6.1
[2.6.0]: https://github.com/joaofelipegalvao/rustodo/compare/v2.5.0...v2.6.0
[2.5.0]: https://github.com/joaofelipegalvao/rustodo/compare/v2.4.1...v2.5.0
[2.4.1]: https://github.com/joaofelipegalvao/rustodo/compare/v2.4.0...v2.4.1
[2.4.0]: https://github.com/joaofelipegalvao/rustodo/compare/v2.3.4...v2.4.0
[2.3.4]: https://github.com/joaofelipegalvao/rustodo/compare/v2.3.3...v2.3.4
[2.3.3]: https://github.com/joaofelipegalvao/rustodo/compare/v2.3.2...v2.3.3
[2.3.2]: https://github.com/joaofelipegalvao/rustodo/compare/v2.3.1...v2.3.2
[2.3.1]: https://github.com/joaofelipegalvao/rustodo/compare/v2.3.0...v2.3.1
[2.3.0]: https://github.com/joaofelipegalvao/rustodo/compare/v2.2.2...v2.3.0
[2.2.2]: https://github.com/joaofelipegalvao/rustodo/compare/v2.2.1...v2.2.2
[2.2.1]: https://github.com/joaofelipegalvao/rustodo/compare/v2.2.0...v2.2.1
[2.2.0]: https://github.com/joaofelipegalvao/rustodo/compare/v2.1.0...v2.2.0
[2.1.0]: https://github.com/joaofelipegalvao/rustodo/compare/v2.0.0...v2.1.0
[2.0.0]: https://github.com/joaofelipegalvao/rustodo/compare/v1.9.0...v2.0.0
[1.9.0]: https://github.com/joaofelipegalvao/rustodo/compare/v1.8.0...v1.9.0
[1.8.0]: https://github.com/joaofelipegalvao/rustodo/compare/v1.7.0...v1.8.0
[1.7.0]: https://github.com/joaofelipegalvao/rustodo/compare/v1.6.0...v1.7.0
[1.6.0]: https://github.com/joaofelipegalvao/rustodo/compare/v1.5.0...v1.6.0
[1.5.0]: https://github.com/joaofelipegalvao/rustodo/compare/v1.4.0...v1.5.0
[1.4.0]: https://github.com/joaofelipegalvao/rustodo/compare/v1.3.0...v1.4.0
[1.3.0]: https://github.com/joaofelipegalvao/rustodo/compare/v1.2.0...v1.3.0
[1.2.0]: https://github.com/joaofelipegalvao/rustodo/compare/v1.1.0...v1.2.0
[1.1.0]: https://github.com/joaofelipegalvao/rustodo/compare/v1.0.0...v1.1.0
[1.0.0]: https://github.com/joaofelipegalvao/rustodo/compare/v0.9.0...v1.0.0
[0.9.0]: https://github.com/joaofelipegalvao/rustodo/compare/v0.8.0...v0.9.0
[0.8.0]: https://github.com/joaofelipegalvao/rustodo/compare/v0.7.0...v0.8.0
[0.7.0]: https://github.com/joaofelipegalvao/rustodo/compare/v0.6.0...v0.7.0
[0.6.0]: https://github.com/joaofelipegalvao/rustodo/compare/v0.5.0...v0.6.0
[0.5.0]: https://github.com/joaofelipegalvao/rustodo/compare/v0.4.2...v0.5.0
[0.4.2]: https://github.com/joaofelipegalvao/rustodo/compare/v0.4.1...v0.4.2
[0.4.1]: https://github.com/joaofelipegalvao/rustodo/compare/v0.4.0...v0.4.1
[0.4.0]: https://github.com/joaofelipegalvao/rustodo/compare/v0.3.0...v0.4.0
[0.3.0]: https://github.com/joaofelipegalvao/rustodo/compare/v0.2.0...v0.3.0
[0.2.0]: https://github.com/joaofelipegalvao/rustodo/compare/v0.1.0...v0.2.0
[0.1.0]: https://github.com/joaofelipegalvao/rustodo/releases/tag/v0.1.0

