#!/bin/bash
set -e

# SceneBrowser Phase 1 GitHub Issues 作成スクリプト

echo "Creating GitHub Issues for SceneBrowser Phase 1..."

# Task #1: Project Setup
echo "Creating Task #1: Project Setup..."
gh issue create \
  --title "Task #1: Project Setup and Initialization" \
  --body-file doc/tasks/01-project-setup.md \
  --label "setup,phase-1" \
  --assignee @me

# Task #2: Database Schema
echo "Creating Task #2: Database Schema..."
gh issue create \
  --title "Task #2: Database Schema and Migrations" \
  --body-file doc/tasks/02-database-schema.md \
  --label "database,phase-1" \
  --assignee @me

# Task #3: Backend Services
echo "Creating Task #3: Backend Services..."
gh issue create \
  --title "Task #3: Backend Core Services Implementation" \
  --body-file doc/tasks/03-backend-services.md \
  --label "backend,services,phase-1" \
  --assignee @me

# Task #4: Tauri Commands
echo "Creating Task #4: Tauri Commands..."
gh issue create \
  --title "Task #4: Tauri Commands Implementation" \
  --body-file doc/tasks/04-tauri-commands.md \
  --label "backend,commands,phase-1" \
  --assignee @me

# Task #5: Frontend Types
echo "Creating Task #5: Frontend Types..."
gh issue create \
  --title "Task #5: Frontend TypeScript Types and Services" \
  --body-file doc/tasks/05-frontend-types.md \
  --label "frontend,typescript,phase-1" \
  --assignee @me

# Task #6: Frontend UI
echo "Creating Task #6: Frontend UI..."
gh issue create \
  --title "Task #6: Frontend UI Components" \
  --body-file doc/tasks/06-frontend-ui.md \
  --label "frontend,ui,react,phase-1" \
  --assignee @me

# Task #7: Integration Testing
echo "Creating Task #7: Integration Testing..."
gh issue create \
  --title "Task #7: Integration and Testing" \
  --body-file doc/tasks/07-integration-testing.md \
  --label "testing,integration,phase-1" \
  --assignee @me

# Task #8: FFmpeg Setup
echo "Creating Task #8: FFmpeg Setup..."
gh issue create \
  --title "Task #8: FFmpeg/FFprobe Setup and Integration" \
  --body-file doc/tasks/08-ffmpeg-setup.md \
  --label "setup,ffmpeg,phase-1" \
  --assignee @me

echo ""
echo "✅ All tasks created successfully!"
echo "View issues at: https://github.com/$(git remote get-url origin | sed 's/.*github.com[:/]\(.*\)\.git/\1/')/issues"
