workflow "clippy-action/ci" {
  on = "push"
  resolves = [
    "clippy-action/ci/lint"
  ]
}

action "clippy-action/ci/lint" {
  uses = "garyttierney/clippy-action@master"
}