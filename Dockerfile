# Use an official Node.js runtime as a parent image
FROM node:16

# Set the working directory in the container
WORKDIR /app

# Copy the package.json and package-lock.json
COPY package*.json ./

# Install any needed dependencies

RUN npm install

# Copy the rest of the project files into the container
COPY . .

# Expose the port that the app will run on
EXPOSE 3000

# Command to run the app
CMD ["node", "index.js"]
