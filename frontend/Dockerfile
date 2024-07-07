FROM node:latest
WORKDIR /vue-setup
RUN npm install -g @vue/cli
WORKDIR /frontend
COPY . .
RUN npm install
EXPOSE 5173
CMD ["npm", "run", "dev", "--host"]