#!/bin/bash

function step {
  STEP_COLOR=33
  echo -ne "\033[0;${STEP_COLOR}m"
  printf %"$(tput cols)"s |tr " " "⚏"
  echo -e " ⚙ $1..."
  printf %"$(tput cols)"s |tr " " "⚏"
  echo -ne "\033[0m"
}

step "Building Frontend"
cd frontend ; npm run build ; cd ..

step "Building Backend"
cd backend ; cargo build ; cd ..
