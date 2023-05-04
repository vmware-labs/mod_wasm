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

/* core/themes/olivero/templates/filter/filter-tips.html.twig */
class __TwigTemplate_0ee0d0c7e087ca01b7842d5b20f0ef0f extends Template
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
        // line 20
        if (($context["multiple"] ?? null)) {
            // line 21
            echo "  <h2>";
            echo $this->extensions['Drupal\Core\Template\TwigExtension']->renderVar(t("Text Formats"));
            echo "</h2>
";
        }
        // line 23
        echo "
";
        // line 24
        if (twig_length_filter($this->env, ($context["tips"] ?? null))) {
            // line 25
            echo "  ";
            if (($context["multiple"] ?? null)) {
                // line 26
                echo "    <div class=\"compose-tips\">
  ";
            }
            // line 28
            echo "
  ";
            // line 29
            $context['_parent'] = $context;
            $context['_seq'] = twig_ensure_traversable(($context["tips"] ?? null));
            foreach ($context['_seq'] as $context["name"] => $context["tip"]) {
                // line 30
                echo "    ";
                if (($context["multiple"] ?? null)) {
                    // line 31
                    echo "      ";
                    // line 32
                    $context["tip_classes"] = [0 => "compose-tips__item", 1 => ("compose-tips__item--name-" . \Drupal\Component\Utility\Html::getClass($this->sandbox->ensureToStringAllowed(                    // line 34
$context["name"], 34, $this->source)))];
                    // line 37
                    echo "      <div";
                    echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, twig_get_attribute($this->env, $this->source, $context["tip"], "attributes", [], "any", false, false, true, 37), "addClass", [0 => ($context["tip_classes"] ?? null)], "method", false, false, true, 37), 37, $this->source), "html", null, true);
                    echo ">
    ";
                }
                // line 39
                echo "    ";
                if ((($context["multiple"] ?? null) || ($context["long"] ?? null))) {
                    // line 40
                    echo "      <h3>";
                    echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, $context["tip"], "name", [], "any", false, false, true, 40), 40, $this->source), "html", null, true);
                    echo "</h3>
    ";
                }
                // line 42
                echo "
    ";
                // line 43
                if (twig_length_filter($this->env, twig_get_attribute($this->env, $this->source, $context["tip"], "list", [], "any", false, false, true, 43))) {
                    // line 44
                    echo "      <ul class=\"filter-tips ";
                    echo $this->extensions['Drupal\Core\Template\TwigExtension']->renderVar(((($context["long"] ?? null)) ? ("filter-tips--long") : ("filter-tips--short")));
                    echo "\">
      ";
                    // line 45
                    $context['_parent'] = $context;
                    $context['_seq'] = twig_ensure_traversable(twig_get_attribute($this->env, $this->source, $context["tip"], "list", [], "any", false, false, true, 45));
                    foreach ($context['_seq'] as $context["_key"] => $context["item"]) {
                        // line 46
                        echo "        ";
                        // line 47
                        $context["item_classes"] = [0 => "filter-tips__item", 1 => ((                        // line 49
($context["long"] ?? null)) ? ("filter-tips__item--long") : ("filter-tips__item--short")), 2 => ((                        // line 50
($context["long"] ?? null)) ? (("filter-tips__item--id-" . \Drupal\Component\Utility\Html::getClass($this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, $context["item"], "id", [], "any", false, false, true, 50), 50, $this->source)))) : (""))];
                        // line 53
                        echo "        <li";
                        echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, twig_get_attribute($this->env, $this->source, $context["item"], "attributes", [], "any", false, false, true, 53), "addClass", [0 => ($context["item_classes"] ?? null)], "method", false, false, true, 53), 53, $this->source), "html", null, true);
                        echo ">";
                        echo $this->extensions['Drupal\Core\Template\TwigExtension']->escapeFilter($this->env, $this->sandbox->ensureToStringAllowed(twig_get_attribute($this->env, $this->source, $context["item"], "tip", [], "any", false, false, true, 53), 53, $this->source), "html", null, true);
                        echo "</li>
      ";
                    }
                    $_parent = $context['_parent'];
                    unset($context['_seq'], $context['_iterated'], $context['_key'], $context['item'], $context['_parent'], $context['loop']);
                    $context = array_intersect_key($context, $_parent) + $_parent;
                    // line 55
                    echo "      </ul>
    ";
                }
                // line 57
                echo "
    ";
                // line 58
                if (($context["multiple"] ?? null)) {
                    // line 59
                    echo "      </div>
    ";
                }
                // line 61
                echo "  ";
            }
            $_parent = $context['_parent'];
            unset($context['_seq'], $context['_iterated'], $context['name'], $context['tip'], $context['_parent'], $context['loop']);
            $context = array_intersect_key($context, $_parent) + $_parent;
            // line 62
            echo "
  ";
            // line 63
            if (($context["multiple"] ?? null)) {
                // line 64
                echo "    </div>
  ";
            }
        }
    }

    public function getTemplateName()
    {
        return "core/themes/olivero/templates/filter/filter-tips.html.twig";
    }

    public function isTraitable()
    {
        return false;
    }

    public function getDebugInfo()
    {
        return array (  144 => 64,  142 => 63,  139 => 62,  133 => 61,  129 => 59,  127 => 58,  124 => 57,  120 => 55,  109 => 53,  107 => 50,  106 => 49,  105 => 47,  103 => 46,  99 => 45,  94 => 44,  92 => 43,  89 => 42,  83 => 40,  80 => 39,  74 => 37,  72 => 34,  71 => 32,  69 => 31,  66 => 30,  62 => 29,  59 => 28,  55 => 26,  52 => 25,  50 => 24,  47 => 23,  41 => 21,  39 => 20,);
    }

    public function getSourceContext()
    {
        return new Source("", "core/themes/olivero/templates/filter/filter-tips.html.twig", "/usr/local/apache2/htdocs/drupal-10-zero/core/themes/olivero/templates/filter/filter-tips.html.twig");
    }
    
    public function checkSecurity()
    {
        static $tags = array("if" => 20, "for" => 29, "set" => 32);
        static $filters = array("t" => 21, "length" => 24, "clean_class" => 34, "escape" => 37);
        static $functions = array();

        try {
            $this->sandbox->checkSecurity(
                ['if', 'for', 'set'],
                ['t', 'length', 'clean_class', 'escape'],
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
