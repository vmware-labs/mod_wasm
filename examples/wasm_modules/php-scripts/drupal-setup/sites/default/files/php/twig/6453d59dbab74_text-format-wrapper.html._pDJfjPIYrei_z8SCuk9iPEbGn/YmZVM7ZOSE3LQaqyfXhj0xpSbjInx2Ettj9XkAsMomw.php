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

/* core/themes/olivero/templates/text-format-wrapper.html.twig */
class __TwigTemplate_d3d8fd878656ed7ae1c7128052e0b814 extends Template
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
        // line 25
        $context["classes"] = [0 => "js-form-item", 1 => "form-item"];
        // line 30
        echo "
";
        // line 31
        echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->extensions['Drupal\Core\Template\TwigExtension']->attachLibrary("olivero/filter.theme"), "html", null, true);
        echo "

<div";
        // line 33
        echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, ($context["attributes"] ?? null), "addClass", [0 => ($context["classes"] ?? null)], "method", false, false, true, 33), 33, $this->source), "html", null, true);
        echo ">
  ";
        // line 34
        echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["children"] ?? null), 34, $this->source), "html", null, true);
        echo "
  ";
        // line 35
        if (($context["description"] ?? null)) {
            // line 36
            echo "    ";
            // line 37
            $context["description_classes"] = [0 => ((            // line 38
($context["aria_description"] ?? null)) ? ("form-item__description") : ("")), 1 => ((            // line 39
($context["disabled"] ?? null)) ? ("is-disabled") : (""))];
            // line 42
            echo "    <div";
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, ($context["description_attributes"] ?? null), "addClass", [0 => ($context["description_classes"] ?? null)], "method", false, false, true, 42), 42, $this->source), "html", null, true);
            echo ">";
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(($context["description"] ?? null), 42, $this->source), "html", null, true);
            echo "</div>
  ";
        }
        // line 44
        echo "</div>
";
    }

    public function getTemplateName()
    {
        return "core/themes/olivero/templates/text-format-wrapper.html.twig";
    }

    public function isTraitable()
    {
        return false;
    }

    public function getDebugInfo()
    {
        return array (  73 => 44,  65 => 42,  63 => 39,  62 => 38,  61 => 37,  59 => 36,  57 => 35,  53 => 34,  49 => 33,  44 => 31,  41 => 30,  39 => 25,);
    }

    public function getSourceContext()
    {
        return new Source("", "core/themes/olivero/templates/text-format-wrapper.html.twig", "/usr/local/apache2/htdocs/drupal-10-zero/core/themes/olivero/templates/text-format-wrapper.html.twig");
    }
    
    public function checkSecurity()
    {
        static $tags = array("set" => 25, "if" => 35);
        static $filters = array("escape" => 31);
        static $functions = array("attach_library" => 31);

        try {
            $this->sandbox->checkSecurity(
                ['set', 'if'],
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
