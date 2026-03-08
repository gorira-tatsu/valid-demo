#!/usr/bin/env bash
set -euo pipefail

ROOT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
BIN_PATH="$ROOT_DIR/target/debug/bss-valid-models"
LOCK_FILE="$ROOT_DIR/valid/contract-lock.json"
OUTPUT_DIR="${1:-/tmp/bss-valid-registry}"

mkdir -p "$OUTPUT_DIR"

log() {
  printf '%s\n' "$*"
}

coverage_transition_percent() {
  sed -n 's/.*"transition_coverage_percent":\([0-9][0-9]*\).*/\1/p' "$1"
}

coverage_decision_percent() {
  sed -n 's/.*"decision_coverage_percent":\([0-9][0-9]*\).*/\1/p' "$1"
}

coverage_guard_percent() {
  sed -n 's/.*"guard_full_coverage_percent":\([0-9][0-9]*\).*/\1/p' "$1"
}

coverage_gate_status() {
  sed -n 's/.*"gate":{"schema_version":"[^"]*","status":"\([^"]*\)".*/\1/p' "$1"
}

properties_for_model() {
  case "$1" in
    board-common-spec)
      printf '%s\n' \
        P_COMMON_HTML_IS_ALWAYS_ESCAPED \
        P_COMMON_BAD_REQUEST_HIDES_INVALID_RESOURCE \
        P_COMMON_WRONG_EDIT_KEY_RETURNS_FORBIDDEN \
        P_COMMON_MISSING_RESOURCE_RETURNS_NOT_FOUND \
        P_COMMON_ANONYMOUS_POST_DEFAULTS_AUTHOR \
        P_COMMON_ANONYMOUS_COMMENT_DEFAULTS_AUTHOR
      ;;
    board-post-list)
      printf '%s\n' \
        P_LIST_PAGE_SIZE_IS_CAPPED_AT_20 \
        P_LIST_EMPTY_STATE_MATCHES_VISIBLE_COUNT \
        P_LIST_DELETED_POSTS_NEVER_SURPASS_TOTAL \
        P_LIST_SUCCESS_EXPOSES_CREATE_NAVIGATION \
        P_LIST_SUCCESS_WITH_POSTS_EXPOSES_DETAIL_NAVIGATION \
        P_LIST_INVALID_PAGE_REQUEST_RETURNS_BAD_REQUEST \
        P_LIST_INVALID_LIMIT_REQUEST_RETURNS_BAD_REQUEST \
        P_LIST_PAGE_OVERFLOW_RETURNS_EMPTY_RESULT
      ;;
    board-post-create)
      printf '%s\n' \
        P_CREATE_SUCCESS_RECORDS_CREATED_AT \
        P_CREATE_SUCCESS_NAVIGATES_TO_DETAIL \
        P_CREATE_FAILURE_PRESERVES_FORM \
        P_CREATE_SUBMITTING_DISABLES_SUBMIT \
        P_CREATE_ANONYMOUS_SUCCESS_DEFAULTS_AUTHOR
      ;;
    board-post-detail)
      printf '%s\n' \
        P_DETAIL_NOT_FOUND_WHEN_POST_IS_UNAVAILABLE \
        P_DETAIL_EMPTY_COMMENT_STATE_MATCHES_COUNT \
        P_DETAIL_VISIBLE_COMMENTS_ARE_SORTED_OLDEST_FIRST \
        P_DETAIL_SUCCESS_EXPOSES_EDIT_ACTIONS \
        P_DETAIL_UPDATED_POST_SHOWS_UPDATED_AT \
        P_DETAIL_UNUPDATED_POST_HIDES_UPDATED_AT
      ;;
    board-edit-delete)
      printf '%s\n' \
        P_EDIT_FORM_IS_PREFILLED_WHEN_OPENED \
        P_EDIT_WRONG_KEY_RETURNS_FORBIDDEN \
        P_DELETE_REQUIRES_CONFIRMATION \
        P_DELETE_HIDES_POST_FROM_VIEWS \
        P_DELETED_POST_CANNOT_BE_REEDITED
      ;;
    board-comment)
      printf '%s\n' \
        P_COMMENT_SUCCESS_RESETS_FORM \
        P_COMMENT_SUCCESS_REFLECTS_ON_DETAIL \
        P_COMMENT_FAILURE_PRESERVES_FORM \
        P_COMMENT_UNAVAILABLE_POST_RETURNS_NOT_FOUND \
        P_COMMENT_VISIBLE_ORDER_IS_OLDEST_FIRST
      ;;
    board-list-rendering)
      printf '%s\n' \
        P_LIST_NEWEST_ORDER_IS_TIMESTAMP_DESCENDING \
        P_LIST_OLDEST_ORDER_IS_TIMESTAMP_ASCENDING \
        P_LIST_NEWEST_ORDER_IS_MONOTONIC_ACROSS_TOP3 \
        P_LIST_OLDEST_ORDER_IS_MONOTONIC_ACROSS_TOP3 \
        P_LIST_NEWEST_TIE_BREAK_IS_STABLE \
        P_LIST_OLDEST_TIE_BREAK_IS_STABLE \
        P_LIST_EXCERPT_WITHIN_LIMIT_IS_NOT_ELLIPSIZED \
        P_LIST_EXCERPT_OVER_LIMIT_IS_120_AND_ELLIPSIZED \
        P_LIST_CONTINUATION_UI_IS_NONE_WITHIN_SINGLE_PAGE \
        P_LIST_CONTINUATION_UI_IS_EXPLICIT_FOR_MULTI_PAGE
      ;;
    board-presentation-contract)
      printf '%s\n' \
        P_PRESENTATION_DATETIME_FORMAT_IS_YYYY_MM_DD_HH_MM \
        P_PRESENTATION_BODY_RENDERING_ESCAPES_HTML_AND_PRESERVES_NEWLINES \
        P_PRESENTATION_SERVER_ERROR_EXPOSES_RETRY_MESSAGE_WITH_PLACEMENT \
        P_PRESENTATION_SUCCESS_MESSAGE_IS_EXPLICIT
      ;;
    board-api-contract)
      printf '%s\n' \
        P_API_RESPONSES_ARE_JSON \
        P_API_LIST_POSTS_RESPONSE_FIELDS_MATCH_CONTRACT \
        P_API_CREATE_POST_RESPONSE_FIELDS_MATCH_CONTRACT \
        P_API_ERROR_RESPONSES_EXPOSE_MESSAGE \
        P_API_BAD_REQUEST_RESPONSES_EXPOSE_FIELD_ERRORS
      ;;
    board-edit-key-storage)
      printf '%s\n' \
        P_EDIT_KEY_STORAGE_NEVER_PERSISTS_PLAINTEXT \
        P_EDIT_KEY_STORAGE_USES_HASH_WHEN_SAVED
      ;;
    board-retry-ux)
      printf '%s\n' \
        P_RETRY_LIST_FAILURE_USES_TOP_BANNER \
        P_RETRY_FORM_FAILURE_USES_BELOW_FORM_MESSAGE \
        P_RETRY_ERROR_STATE_IS_ACTIONABLE \
        P_RETRY_RECOVERY_CLEARS_ERROR_MESSAGE \
        P_RETRY_RECOVERY_DOES_NOT_KEEP_STALE_SERVER_ERROR
      ;;
    board-submission-discipline)
      printf '%s\n' \
        P_SUBMIT_IN_FLIGHT_DISALLOWS_SECOND_SUBMIT \
        P_SUBMIT_DUPLICATE_ATTEMPT_IS_BLOCKED \
        P_SUBMIT_FAILURE_ENSURES_RETRYABLE_FORM \
        P_SUBMIT_SUCCESS_OR_RECOVERY_REENABLES_NORMAL_FLOW
      ;;
    board-message-contract)
      printf '%s\n' \
        P_MESSAGE_EMPTY_LIST_IS_BOUND_TO_LIST_SCREEN \
        P_MESSAGE_POST_CREATED_COMPLETED_IS_BOUND_TO_CREATE_SUCCESS \
        P_MESSAGE_INVALID_EDIT_KEY_IS_BOUND_TO_FORBIDDEN_EDIT
      ;;
    board-flow)
      printf '%s\n' \
        P_FLOW_DELETED_POST_IS_HIDDEN_FROM_LIST \
        P_FLOW_DELETED_POST_IS_HIDDEN_FROM_DETAIL \
        P_FLOW_DELETED_POST_DISALLOWS_COMMENT \
        P_FLOW_VISIBLE_DETAIL_REQUIRES_LIST_NAVIGATION \
        P_FLOW_LIST_AND_DETAIL_COMMENT_COUNTS_MATCH \
        P_FLOW_UPDATED_POST_SHOWS_UPDATED_AT_ON_DETAIL \
        P_FLOW_UPDATED_POST_KEEPS_LIST_AND_DETAIL_CONTENT_ALIGNED \
        P_FLOW_DETAIL_LOAD_FAILURE_EXPOSES_RETRY_AND_RETURN_PATH \
        P_FLOW_DETAIL_RECOVERY_RESTORES_UPDATED_AND_COMMENT_CONSISTENCY
      ;;
    *)
      return 1
      ;;
  esac
}

MODELS=(
  board-common-spec
  board-post-list
  board-post-create
  board-post-detail
  board-edit-delete
  board-comment
  board-list-rendering
  board-presentation-contract
  board-api-contract
  board-edit-key-storage
  board-retry-ux
  board-submission-discipline
  board-message-contract
  board-flow
)

log "Building registry binary"
cargo build --manifest-path "$ROOT_DIR/Cargo.toml"

if [[ ! -x "$BIN_PATH" ]]; then
  log "registry binary not found: $BIN_PATH"
  exit 1
fi

log "Checking contract lock"
"$BIN_PATH" contract check "$LOCK_FILE" --json >"$OUTPUT_DIR/contract-check.json"
if rg -q '"status":"changed"' "$OUTPUT_DIR/contract-check.json"; then
  log "contract drift detected"
  exit 1
fi

declare -a coverage_warnings=()

for model in "${MODELS[@]}"; do
  model_dir="$OUTPUT_DIR/$model"
  mkdir -p "$model_dir"

  log "Linting $model"
  "$BIN_PATH" lint "$model" --json >"$model_dir/lint.json"
  if ! rg -q '"status":"ok"' "$model_dir/lint.json"; then
    log "lint failed for $model"
    exit 1
  fi

  while IFS= read -r property; do
    [[ -n "$property" ]] || continue
    log "Checking $model :: $property"
    "$BIN_PATH" check "$model" "--property=$property" --json >"$model_dir/$property.json"
    if ! rg -q '"status":"PASS"' "$model_dir/$property.json"; then
      log "property failed for $model :: $property"
      exit 1
    fi
  done < <(properties_for_model "$model")

  log "Collecting coverage $model"
  "$BIN_PATH" coverage "$model" --json >"$model_dir/coverage.json"
  transition="$(coverage_transition_percent "$model_dir/coverage.json")"
  decision="$(coverage_decision_percent "$model_dir/coverage.json")"
  guard="$(coverage_guard_percent "$model_dir/coverage.json")"
  gate="$(coverage_gate_status "$model_dir/coverage.json")"
  log "Coverage $model :: transition=${transition}% decision=${decision}% guard=${guard}% gate=${gate}"
  if [[ "$gate" != "pass" ]]; then
    coverage_warnings+=("$model transition=${transition}% decision=${decision}% guard=${guard}% gate=${gate}")
  fi
done

if (( ${#coverage_warnings[@]} > 0 )); then
  log ""
  log "Coverage warnings:"
  for warning in "${coverage_warnings[@]}"; do
    log "  $warning"
  done
  log "Detailed JSON reports: $OUTPUT_DIR"
  exit 0
fi

log "All contract, lint, property, and coverage checks passed"
