FROM node:18-alpine3.15

ARG next_public_base_api_url
ENV NEXT_PUBLIC_BASE_API_URL=$next_public_base_api_url

RUN mkdir -p /root/app
WORKDIR /root/app

COPY package.json yarn.lock /root/app/
RUN yarn install
COPY . /root/app/
RUN yarn build

CMD ["yarn", "start"]
