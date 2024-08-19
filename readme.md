<samp>

[DRAFT]

Cross-platform functional language for powerful native mobile apps

A simple, efficient, and powerful programming language designed for native mobile development on iOS and Android. This language aims to streamline the mobile development process with a focus on clarity, performance, and cross-platform capabilities.

Key Features:

- Cross-platform: Write once, run natively on both iOS and Android.
- Functional: Emphasizes functional programming principles for clean, maintainable code.
- Simplicity: Minimalist syntax to maximize developer productivity.
- Efficiency: Compiles to optimized native code for both platforms.
- Native API Integration: Seamlessly interacts with native mobile APIs.
- Centralized Configuration: Manage routes and middleware in a single config file for easy optimization.

Basic Photo Capture and Save:

```rs
import camera from "os/camera"
import storage from "os/storage"

fn takePhotoAndSave(): void {
    let photo = camera.capture()
    storage.save(photo, "photo.jpg")
}

takePhotoAndSave()
```

Functional Component with State:

```rs
import View from "ui/view"
import Text from "ui/text"
import Button from "ui/button"

fn Counter(): View {
  let count = state(0)
      return (
          <View>
              <Text>Count: {count}</Text>
              <Button onPress={() => count.set(count.get() + 1)}>
                  Increment
              </Button>
          </View>
      )
}

```

Centralized Route Configuration:

```rs
routes = {
    home: {
        component: "screens/HomeScreen",
        middlewares: ["authCheck"],
    },
    profile: {
        component: "screens/ProfileScreen",
        middlewares: ["authCheck", "loadUserData"],
    },
}

middlewares = {
    authCheck: fn(): bool {
        return auth.isAuthenticated()
    },
    loadUserData: fn(userId: Int): User {
        return api.getUserData(userId)
    },
}
```

Navigation with Middleware:

```rs
fn navigate(route: string, params: dict): void {
    let routeConfig = config.routes[route]

    // Execute middlewares before navigating
    for (middleware in routeConfig.middlewares) {
        if (!middlewares[middleware](params)) {
            Navigator.to(<ErrorScreen message="Unauthorized" />)
            return
        }
    }

    // Navigate to the route's component
    Navigator.to(routeConfig.component, params)
}

navigate("profile", { userId: 42 })

```
