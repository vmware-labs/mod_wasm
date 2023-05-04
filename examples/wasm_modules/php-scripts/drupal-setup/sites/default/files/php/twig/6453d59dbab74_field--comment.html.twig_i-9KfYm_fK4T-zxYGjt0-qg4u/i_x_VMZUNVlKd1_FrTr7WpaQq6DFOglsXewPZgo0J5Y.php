<?php

use Twig\Environment;
use Twig\Error\LoaderError;
use Twig\Error\RuntimeError;
use Twig\Extension\SandboxExtension;
use Twig\Markup;
use Twig\Sandbox\SecurityError;
use Twig\Sandbox\SecurityNotAllowedTagError;
use Twig\Sandbox\SecurityNotAllowedFilterError;
use Twig\Sandbox\SecurityNotAllowedFunctionError;
use Twig\Source;
use Twig\Template;

/* core/themes/olivero/templates/field/field--comment.html.twig */
class __TwigTemplate_b46187764566712d809f5b62176a9c2f extends Template
{
    private $source;
    private $macros = [];

    public function __construct(Environment $env)
    {
        parent::__construct($env);

        $this->source = $this->getSourceContext();

        $this->parent = false;

        $this->blocks = [
        ];
        $this->sandbox = $this->env->getExtension('\Twig\Extension\SandboxExtension');
        $this->checkSecurity();
    }

    protected function doDisplay(array $context, array $blocks = [])
    {
        $macros = $this->macros;
        // line 30
        echo "
";
        // line 31
        echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->extensions['Drupal\Core\Template\TwigExtension']->attachLibrary("olivero/comments"), "html", null, true);
        echo "
<section";
        // line 32
        echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, twig_get_attribute($this->env, $this->source, ($context["attributes"] ?? null), "setAttribute", [0 => "data-drupal-selector", 1 => "comments"], "method", false, false, true, 32), "addClass", [0 => "comments"], "method", false, false, true, 32), 32, $this->source), "html", null, true);
        echo ">

  ";
        // line 34
        if ( !($context["label_hidden"] ?? null)) {
            // line 35
            echo "    ";
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["title_prefix"] ?? null), 35, $this->source), "html", null, true);
            echo "
    <h2";
            // line 36
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, ($context["title_attributes"] ?? null), "addClass", [0 => "comments__title"], "method", false, false, true, 36), 36, $this->source), "html", null, true);
            echo ">";
            // line 37
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["label"] ?? null), 37, $this->source), "html", null, true);
            // line 38
            if (($context["comments"] ?? null)) {
                // line 39
                echo "<span class=\"comments__count\">";
                echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["comment_count"] ?? null), 39, $this->source), "html", null, true);
                echo "</span>";
            }
            // line 41
            echo "</h2>
    ";
            // line 42
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["title_suffix"] ?? null), 42, $this->source), "html", null, true);
            echo "
  ";
        }
        // line 44
        echo "
  ";
        // line 45
        if (($context["comment_form"] ?? null)) {
            // line 46
            echo "    <div class=\"add-comment\">
      ";
            // line 47
            if (($context["user_picture"] ?? null)) {
                // line 48
                echo "      <div class=\"add-comment__picture-wrapper\">
        <div class=\"add-comment__picture\">
          ";
                // line 50
                echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["user_picture"] ?? null), 50, $this->source), "html", null, true);
                echo "
        </div>
      </div>
      ";
            }
            // line 54
            echo "      <div class=\"add-comment__form\">
        ";
            // line 55
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["comment_form"] ?? null), 55, $this->source), "html", null, true);
            echo "
      </div>
    </div>
  ";
        }
        // line 59
        echo "
  ";
        // line 60
        echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["comments"] ?? null), 60, $this->source), "html", null, true);
        echo "

</section>
";
    }

    public function getTemplateName()
    {
        return "core/themes/olivero/templates/field/field--comment.html.twig";
    }

    public function isTraitable()
    {
        return false;
    }

    public function getDebugInfo()
    {
        return array (  112 => 60,  109 => 59,  102 => 55,  99 => 54,  92 => 50,  88 => 48,  86 => 47,  83 => 46,  81 => 45,  78 => 44,  73 => 42,  70 => 41,  65 => 39,  63 => 38,  61 => 37,  58 => 36,  53 => 35,  51 => 34,  46 => 32,  42 => 31,  39 => 30,);
    }

    public function getSourceContext()
    {
        return new Source("", "core/themes/olivero/templates/field/field--comment.html.twig", "/usr/local/apache2/htdocs/drupal-10-zero/core/themes/olivero/templates/field/field--comment.html.twig");
    }
    
    public function checkSecurity()
    {
        static $tags = array("if" => 34);
        static $filters = array("escape" => 31);
        static $functions = array("attach_library" => 31);

        try {
            $this->sandbox->checkSecurity(
                ['if'],
                ['escape'],
                ['attach_library']
            );
        } catch (SecurityError $e) {
            $e->setSourceContext($this->source);

            if ($e instanceof SecurityNotAllowedTagError && isset($tags[$e->getTagName()])) {
                $e->setTemplateLine($tags[$e->getTagName()]);
            } elseif ($e instanceof SecurityNotAllowedFilterError && isset($filters[$e->getFilterName()])) {
                $e->setTemplateLine($filters[$e->getFilterName()]);
            } elseif ($e instanceof SecurityNotAllowedFunctionError && isset($functions[$e->getFunctionName()])) {
                $e->setTemplateLine($functions[$e->getFunctionName()]);
            }

            throw $e;
        }

    }
}
