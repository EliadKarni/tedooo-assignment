# Use the official OpenJDK image as the base
FROM openjdk:11

# Install Android SDK and other dependencies
RUN apt-get update && apt-get install -y wget unzip && \
    wget https://dl.google.com/android/repository/commandlinetools-linux-9477386_latest.zip -O cmdline-tools.zip && \
    unzip cmdline-tools.zip -d /sdk && \
    rm cmdline-tools.zip

# Set environment variables
ENV ANDROID_HOME=/sdk
ENV PATH=$ANDROID_HOME/cmdline-tools/bin:$PATH

# Set the working directory
WORKDIR /app

# Copy the Android project files
COPY . .

# Build the Android project
RUN ./gradlew assembleDebug

# Expose any necessary ports (if applicable)
EXPOSE 8081

# Default command
CMD ["./gradlew", "assembleDebug"]