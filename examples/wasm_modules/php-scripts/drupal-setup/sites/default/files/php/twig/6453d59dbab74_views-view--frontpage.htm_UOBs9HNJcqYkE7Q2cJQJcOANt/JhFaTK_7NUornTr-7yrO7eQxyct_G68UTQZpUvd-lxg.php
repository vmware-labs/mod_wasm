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

/* core/themes/olivero/templates/views/views-view--frontpage.html.twig */
class __TwigTemplate_0f6a737c6833d53e15697ad92c849f35 extends Template
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
        // line 35
        echo "
";
        // line 37
        $context["classes"] = [0 => "view", 1 => ("view-" . \Drupal\Component\Utility\Html::getClass($this->sandbox->ensureToStringAllowed(        // line 39
($context["id"] ?? null), 39, $this->source))), 2 => ("view-id-" . $this->sandbox->ensureToStringAllowed(        // line 40
($context["id"] ?? null), 40, $this->source)), 3 => ("view-display-id-" . $this->sandbox->ensureToStringAllowed(        // line 41
($context["display_id"] ?? null), 41, $this->source)), 4 => "grid-full", 5 => "layout--pass--content-narrow"];
        // line 46
        echo "<div";
        echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, ($context["attributes"] ?? null), "addClass", [0 => ($context["classes"] ?? null)], "method", false, false, true, 46), 46, $this->source), "html", null, true);
        echo ">
  ";
        // line 47
        echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["title_prefix"] ?? null), 47, $this->source), "html", null, true);
        echo "
  ";
        // line 48
        if (($context["title"] ?? null)) {
            // line 49
            echo "    ";
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["title"] ?? null), 49, $this->source), "html", null, true);
            echo "
  ";
        }
        // line 51
        echo "  ";
        echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["title_suffix"] ?? null), 51, $this->source), "html", null, true);
        echo "
  ";
        // line 52
        if (($context["header"] ?? null)) {
            // line 53
            echo "    <div class=\"view-header\">
      ";
            // line 54
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["header"] ?? null), 54, $this->source), "html", null, true);
            echo "
    </div>
  ";
        }
        // line 57
        echo "  ";
        if (($context["exposed"] ?? null)) {
            // line 58
            echo "    <div class=\"view-filters\">
      ";
            // line 59
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["exposed"] ?? null), 59, $this->source), "html", null, true);
            echo "
    </div>
  ";
        }
        // line 62
        echo "  ";
        if (($context["attachment_before"] ?? null)) {
            // line 63
            echo "    <div class=\"attachment attachment-before\">
      ";
            // line 64
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["attachment_before"] ?? null), 64, $this->source), "html", null, true);
            echo "
    </div>
  ";
        }
        // line 67
        echo "
  ";
        // line 68
        if (($context["rows"] ?? null)) {
            // line 69
            echo "    <div class=\"view-content\">
      ";
            // line 70
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["rows"] ?? null), 70, $this->source), "html", null, true);
            echo "
    </div>
  ";
        } elseif (        // line 72
($context["empty"] ?? null)) {
            // line 73
            echo "    ";
            $this->loadTemplate("@olivero/includes/get-started.html.twig", "core/themes/olivero/templates/views/views-view--frontpage.html.twig", 73)->display($context);
            // line 74
            echo "  ";
        }
        // line 75
        echo "
  ";
        // line 76
        if (($context["pager"] ?? null)) {
            // line 77
            echo "    ";
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["pager"] ?? null), 77, $this->source), "html", null, true);
            echo "
  ";
        }
        // line 79
        echo "  ";
        if (($context["attachment_after"] ?? null)) {
            // line 80
            echo "    <div class=\"attachment attachment-after\">
      ";
            // line 81
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["attachment_after"] ?? null), 81, $this->source), "html", null, true);
            echo "
    </div>
  ";
        }
        // line 84
        echo "  ";
        if (($context["more"] ?? null)) {
            // line 85
            echo "    ";
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["more"] ?? null), 85, $this->source), "html", null, true);
            echo "
  ";
        }
        // line 87
        echo "  ";
        if (($context["footer"] ?? null)) {
            // line 88
            echo "    <div class=\"view-footer\">
      ";
            // line 89
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["footer"] ?? null), 89, $this->source), "html", null, true);
            echo "
    </div>
  ";
        }
        // line 92
        echo "
</div>
";
    }

    public function getTemplateName()
    {
        return "core/themes/olivero/templates/views/views-view--frontpage.html.twig";
    }

    public function isTraitable()
    {
        return false;
    }

    public function getDebugInfo()
    {
        return array (  169 => 92,  163 => 89,  160 => 88,  157 => 87,  151 => 85,  148 => 84,  142 => 81,  139 => 80,  136 => 79,  130 => 77,  128 => 76,  125 => 75,  122 => 74,  119 => 73,  117 => 72,  112 => 70,  109 => 69,  107 => 68,  104 => 67,  98 => 64,  95 => 63,  92 => 62,  86 => 59,  83 => 58,  80 => 57,  74 => 54,  71 => 53,  69 => 52,  64 => 51,  58 => 49,  56 => 48,  52 => 47,  47 => 46,  45 => 41,  44 => 40,  43 => 39,  42 => 37,  39 => 35,);
    }

    public function getSourceContext()
    {
        return new Source("", "core/themes/olivero/templates/views/views-view--frontpage.html.twig", "/usr/local/apache2/htdocs/drupal-10-zero/core/themes/olivero/templates/views/views-view--frontpage.html.twig");
    }
    
    public function checkSecurity()
    {
        static $tags = array("set" => 37, "if" => 48, "include" => 73);
        static $filters = array("clean_class" => 39, "escape" => 46);
        static $functions = array();

        try {
            $this->sandbox->checkSecurity(
                ['set', 'if', 'include'],
                ['clean_class', 'escape'],
                []
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
