#!/usr/bin/env bash

function main() {
    local branches=(
        '00-init           00 - Initialisation'
        '01-blueprint      01 - Blueprint'
        '02-impl-default   02 - impl Default'
        '03-modules        03 - Modules'
        '04-debug          04 - Debug'
        '05-attribute      05 - Attribut'
        '06-errors         06 - Gestion des erreurs'
    )

    local push_args=()
    local branch=''
    local branch_name=''
    local branch_message=''
    local commit=''
    i=0
    while [[ $i < ${#branches[@]} ]]; do
        branch="${branches[i]}"
        branch_name="$(sed -Ee 's/^([^ ]+).*/\1/' <<<"${branch}")" &&
        branch_message="$(sed -Ee 's/^[^ ]+ *//' <<<"${branch}")" &&
        branch_message="${branch_message##*( )}" &&
        commit="$(git log '--pretty=format:%H' --reverse --grep "${branch_message}" --fixed-strings --max-count '1' 99-final)" &&
        true || return $?
        [[ ! -z "${commit}" ]] || {
            echo "No commit found for branch '${branch_name}' with message '${branch_message}'" >&2
            return 1
        }
        printf '%40s  %-15s  %s\n' "${commit}"  "${branch_name}"  "${branch_message}"
        push_args+=( "${commit}:refs/heads/${branch_name}" )
        i=$((i + 1))
    done
    echo

    echo git push origin --force-with-lease "${push_args[@]}"
}

main "$@"
