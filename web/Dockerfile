FROM node AS build

WORKDIR /app
COPY package*.json ./
RUN npm install
COPY . .
RUN npm run build

FROM nginx:stable-alpine

COPY --from=build /app/public /usr/share/nginx/html
