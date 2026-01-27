# Usernames in crates.io

The user-facing identifier for user accounts in crates.io is currently always equal to the GitHub username sent via OAuth, which is in the database in `users.gh_login`.

## Current uses of `users.gh_login`

- When visiting `https://crates.io/users/{username}`, which makes a request to `https://crates.io/api/v1/users/{username}`, the username is normalized to all lowercase and used to find a user profile to display by querying with `lower(gh_login)`
- In API responses containing ownership information: `EncodableOwner.login` (and also `EncodableOwner.url`, this needs to continue using GitHub usernames for links to github.com), and similarly for `EncodablePublicUser` and `EncodablePrivateUser`
- To specify which user you would like to invite to become an owner of a crate or remove as an owner of a crate
- In Open Graph images for crates, when displaying owners
- In emails identifying users who published crates, deleted crates, added an email address to be verified, created API tokens, has an expired API token, sent or received an invite to become an owner, or changed trusted publishing configuration
- In log messages identifying the user who took a particular action
- When checking if the currently-logged-in user is a member of a team owner of a crate, to determine whether the currently-logged-in user has permission to publish a crate or permission to add this team as an owner (this needs to continue using GitHub usernames for GitHub teams)
- The admin API endpoint `GET /api/private/admin_list/{users.gh_login}` that I use to identify empty crates that don't contain functionality, contrary to our squatting policy.
- The admin tool that checks what user a token belongs to
- The admin tool that deletes crates displays the crate's owners' usernames for verification
- The admin tool that checks to see if potential typosquats have the same owners or not

## Possibilities

- Discord-like IDs that incorporate crates.io's `users.id`
- Disambiguation pages when there are multiple users with the exact same username (what about no longer valid/renamed/deleted github accounts?)
- Display "GitHub user `{users.gh_login}`" everywhere `gh_login` is currently displayed
- Create the concept of a crates.io username that may or may not be the same as the username from any linked services

## Possible attacks/vulnerabilities

Confusion with a person who has a higher reputation to gain trust is the highest risk.

Impersonation could occur via:

- If we were to add GitLab OAuth, and Person A held `well_known_username` on GitHub and crates.io, and Person B registered `well_known_username` on GitLab and crates.io
- If we were to add GitLab OAuth, and Person A held `well_known_username` on GitHub _but had never logged in to crates.io_, and Person B registered `well_known_username` on GitLab and crates.io

Example uses of impersonation to carry out attacks:

  - Asks to be added as an owner with a username that appears to be someone else
  - Publishes a crate that appears to be owned by someone else

## Other notes

Currently, on a crate's page, we display the owners with:

```
<a href="https://crates.io/users/{{ owner.login }}">
    <img src="{{ owner.avatar }}" alt="{{ owner.name }} ({{ owner.login }})" />
    {{ owner.name ? owner.login }}
</a>
```

- It takes one query to get this info; with `oauth_github` and other tables it'll need 2+
  - If there are multiple owners, we can query for all github oauth for all owners at once and then re-collate? So that at least it's only N+1 queries, not N*M+1
- You can already sort of impersonate someone by changing your display name and avatar in github to someone else's display name and avatar, not sure how quickly github would act on that
