from django.contrib.auth.forms import AuthenticationForm, UserCreationForm
from django.contrib.auth.models import User
from django.db.models.fields import forms

from crispy_forms.helper import FormHelper 
from crispy_forms.layout import Fieldset, Layout, Field, Submit

class RegisterForm(UserCreationForm):
    username = forms.CharField(label="Username")
    password1 = forms.CharField(label="Password", widget=forms.PasswordInput)
    password2 = forms.CharField(label="Repeat Password", widget=forms.PasswordInput)

    class Meta(UserCreationForm.Meta):
        model = User
        fields = ['username', 'password1', 'password2']

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.helper = FormHelper(self)
        self.helper.form_class = 'audio-form'
        self.helper.layout = Layout(
            Fieldset(
                "Register to AudioNote",
                Field('username'),
                Field('password1'),
                Field('password2'),
                Submit('submit', "Log in", css_class='form-button'),
                css_class="form-fields-custom"
            )
        )


class LoginForm(AuthenticationForm):
    username = forms.CharField(label="Username")
    password = forms.CharField(label="Password", widget=forms.PasswordInput)

    def __init__(self, request=None, *args, **kwargs):
        super().__init__(request, *args, **kwargs)
        self.helper = FormHelper(self)
        self.helper.form_class = 'audio-form'
        self.helper.layout = Layout(
            Fieldset(
                'Log in to AudioNote',
                Field('username', css_class="form-input"),
                Field('password', css_class="form-field"),
                Submit('submit', "Log in", css_class='form-button'),
                css_class="form-fields-custom"
            ),
        )
