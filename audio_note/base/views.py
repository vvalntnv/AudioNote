from django.urls import reverse_lazy
from django.views.generic import FormView, TemplateView

from base.forms import RegistrationForm

# Create your views here.
class HomeView(TemplateView):
    template_name = "base/home.htmldjango"

class RegisterView(FormView):
    template_name = "base/register.htmldjango"
    form_class = RegistrationForm
    success_url = reverse_lazy("home")
