# hooks [![Build Status](https://github.com/based-zrt/hooks/actions/workflows/build.yml/badge.svg)](https://github.com/based-zrt/hooks/actions/workflows/build.yml)

A small webhook transformation tool.

### What?

Receive webhook pushes from Docker Registries or Jira, and forward them to Discord Webhooks (with neat embeds)

## Docker Compose

> Example Docker Compose setup:

```yml
services:
  hooks:
    image: <your image name>
    container_name: hooks
    restart: unless-stopped
    environment:
      JIRA_TOKEN: "" # URL query field for authenticating the jira post request
      JIRA_URL: "" # Discord webhook URL for jira messages
      DOCKER_TOKEN: "" # Bearer token for Docker Registry post request
      DOCKER_URL: "" # Discord webhook URL for docker messages
      # In order to deliver issue type & project avatars, we have to download and redirect the images using an authorized
      JIRA_EMAIL: "" # account email
      JIRA_REST_TOKEN: "" # account rest api token
      HOST_URL: "" # the base url for the service
    volumes:
      - /path/to/images:/app/img
      - /path/to/requests:/app/requests
```

## License

```
    hooks
    Copyright (C) 2023  SunStorm

    This program is free software: you can redistribute it and/or modify
    it under the terms of the GNU Affero General Public License as published
    by the Free Software Foundation, either version 3 of the License, or
    (at your option) any later version.

    This program is distributed in the hope that it will be useful,
    but WITHOUT ANY WARRANTY; without even the implied warranty of
    MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
    GNU Affero General Public License for more details.

    You should have received a copy of the GNU Affero General Public License
    along with this program.  If not, see <https://www.gnu.org/licenses/>.
```
