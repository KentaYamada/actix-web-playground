FROM node:18-alpine3.19

ENV LANG jp_JP.UTF-8
ENV TZ Asia/Tokyo

WORKDIR /app

COPY ./package.json ./package-lock.json ./

RUN apk update && apk add xdg-utils && npm install
