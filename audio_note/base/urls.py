from django.urls import path

from base.views import HomeView

urlpatterns = [
    path("", HomeView.as_view(), name="home")
]
